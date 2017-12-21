module Msgs exposing (..)

import Models exposing (Index)
import RemoteData exposing (WebData)


type Msg
    = OnFetchIndices (WebData (List Index))