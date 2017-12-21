module Models exposing (..)


type alias Model =
    { indices : List Index
    }


initialModel : Model
initialModel =
    { indices = [ Index "1" "Test"]
    }


type alias IndexId =
    String

type alias Index =
    { id : IndexId
    , name : String
    }
