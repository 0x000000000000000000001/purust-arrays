module Test.Extra where

import Prelude

import Control.Monad.ST as ST
import Data.Array as Array
import Data.Array.ST as STA
import Data.Array.ST.Iterator as Iterator
import Data.Maybe (Maybe(..))
import Data.Unfoldable (replicateA)
import Effect (Effect, foreachE)
import Effect.Console (log)
import Effect.Ref as Ref
import Test.Assert (assert, assertEqual)
import Test.Probe (opaque)

run :: Effect Unit
run = do
  log "Additional: captured callbacks and records across an opaque FFI boundary"
  let
    offset = opaque 10
    input = opaque [1, 2, 3]
    callback = opaque (\n -> { value: n + offset, label: "kept" })
    records = map callback input
  assertEqual
    { actual: records
    , expected: [{ value: 11, label: "kept" }, { value: 12, label: "kept" }, { value: 13, label: "kept" }]
    }
  assertEqual { actual: Array.filter (opaque (\r -> r.value > 11)) records, expected: Array.drop 1 records }
  assertEqual { actual: Array.zipWith (opaque (\a b -> a * 10 + b)) input (opaque [4, 5]), expected: [14, 25] }
  assertEqual { actual: input, expected: [1, 2, 3] }

  log "Additional: immutable snapshots and independent ST allocations"
  assert $ ST.run do
    let allocate = STA.new
    first <- allocate
    second <- allocate
    void $ STA.push 1 first
    untouched <- STA.freeze second
    snapshot <- STA.freeze first
    clone <- STA.clone first
    void $ STA.poke 0 2 first
    current <- STA.freeze first
    cloned <- STA.freeze clone
    pure $ untouched == ([] :: Array Int) && snapshot == [1] && current == [2] && cloned == [1]

  log "Additional: ST actions execute only when run and can be replayed"
  assert $ ST.run do
    array <- STA.thaw [1]
    let append = STA.push 2 array
    before <- STA.freeze array
    first <- append
    second <- append
    result <- STA.freeze array
    pure $ before == [1] && first == 2 && second == 3 && result == [1, 2, 2]

  log "Additional: checked ST bounds preserve the array"
  assert $ ST.run do
    array <- STA.thaw [1, 2]
    left <- STA.peek (-1) array
    right <- STA.peek 2 array
    wroteLeft <- STA.poke (-1) 99 array
    wroteRight <- STA.poke 2 99 array
    result <- STA.freeze array
    pure $ left == Nothing && right == Nothing && not wroteLeft && not wroteRight && result == [1, 2]

  log "Additional: iterator peek, next, pushWhile, pushAll and iterate"
  assert $ ST.run do
    iterator <- Iterator.iterator (Array.index [1, 2, 3])
    first <- Iterator.peek iterator
    repeated <- Iterator.peek iterator
    consumed <- Iterator.next iterator
    output <- STA.new
    Iterator.pushWhile (_ < 3) iterator output
    middle <- STA.freeze output
    remaining <- Iterator.peek iterator
    Iterator.pushAll iterator output
    exhausted <- Iterator.exhausted iterator
    values <- STA.freeze output
    iterator2 <- Iterator.iterator (Array.index [1, 2, 3])
    output2 <- STA.new
    Iterator.iterate iterator2 (\n -> void $ STA.push (n * 10) output2)
    mapped <- STA.freeze output2
    pure $ first == Just 1 && repeated == Just 1 && consumed == Just 1
      && middle == [2] && remaining == Just 3 && exhausted
      && values == [2, 3] && mapped == [10, 20, 30]

  log "Additional: replicateA effect count, deferred execution and replay"
  foreachE [-1, 0, 1, 3, 100000] \size -> do
    count <- Ref.new 0
    let action = replicateA size do
          Ref.modify_ (_ + 1) count
          Ref.read count
    Ref.read count >>= \actual -> assertEqual { actual, expected: 0 }
    foreachE [1, 2] \_ -> do
      Ref.write 0 count
      result <- action
      assertEqual { actual: Array.length result, expected: max 0 size }
      when (size > 0) $ assertEqual { actual: result, expected: Array.range 1 size }
      Ref.read count >>= \actual -> assertEqual { actual, expected: max 0 size }
  log "Additional Array checks passed"
