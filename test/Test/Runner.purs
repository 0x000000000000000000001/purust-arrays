module Test.Runner where

import Prelude
import Effect (Effect)
import Test.Main as Original
import Test.Extra as Extra

main :: Effect Unit
main = do
  Original.main
  Extra.run
