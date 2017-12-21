module Indices.List exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class)
import Msgs exposing (Msg)
import Models exposing (Index)


view : List Index -> Html Msg
view indices =
    div []
        [ nav
        , list indices
        ]


nav : Html Msg
nav =
    div [ class "clearfix mb2 white bg-black" ]
        [ div [ class "left p2" ] [ text "Indices" ] ]


list : List Index -> Html Msg
list indices =
    div [ class "p2" ]
        [ table []
            [ thead []
                [ tr []
                    [ th [] [ text "Id" ]
                    , th [] [ text "Name" ]
                    ]
                ]
            , tbody [] (List.map indexRow indices)
            ]
        ]


indexRow : Index -> Html Msg
indexRow index =
    tr []
        [ td [] [ text index.id ]
        , td [] [ text index.name ]
        , td []
            []
        ]