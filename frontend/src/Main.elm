module Main exposing (..)

import Browser
import Html exposing (..)
import Html.Attributes as Attributes exposing (style)
import Html.Events as Events exposing (onInput)
import Http
import Json.Decode as Decode exposing (Decoder, float, int, list, string, succeed)
import Json.Decode.Pipeline exposing (optional, required)



---- MODEL ----


type alias ProductInputBase a =
    { a
        | name : String
        , stock : Float
        , price : Maybe Int
    }


setProductInputName : String -> ProductInputBase a -> ProductInputBase a
setProductInputName name productInput =
    { productInput | name = name }


asNameInProductInput : ProductInputBase a -> String -> ProductInputBase a
asNameInProductInput productInput name =
    { productInput | name = name }


setProductInputStock : Float -> ProductInputBase a -> ProductInputBase a
setProductInputStock stock productInput =
    { productInput | stock = stock }


asStockInProductInput : ProductInputBase a -> Float -> ProductInputBase a
asStockInProductInput productInput stock =
    { productInput | stock = stock }


setProductInputPrice : Maybe Int -> ProductInputBase a -> ProductInputBase a
setProductInputPrice price productInput =
    { productInput | price = price }


asPriceInProductInput : ProductInputBase a -> Maybe Int -> ProductInputBase a
asPriceInProductInput productInput price =
    { productInput | price = price }


type alias ProductInput =
    ProductInputBase {}


type alias Product =
    ProductInputBase
        { id : Int
        , user_id : Int
        }


setProductId product id =
    { product | id = id }


setProductUserId product user_id =
    { product | user_id = user_id }


emptyProductInput : ProductInput
emptyProductInput =
    { name = ""
    , stock = 0
    , price = Just 0
    }


createProduct : String -> Float -> Maybe Int -> Int -> Int -> Product
createProduct name stock price id user_id =
    { name = name
    , stock = stock
    , price = price
    , id = id
    , user_id = user_id
    }


emptyProduct : Product
emptyProduct =
    createProduct "" 0 Nothing 0 0


productDecoder : Decoder Product
productDecoder =
    Decode.succeed createProduct
        |> required "name" string
        |> required "stock" float
        |> optional "price" (Decode.nullable int) Nothing
        |> required "id" int
        |> required "user_id" int


type alias AppData =
    { products : List Product
    , currentEditing : Maybe Product
    , newProductData : ProductInput
    }


emptyAppData : AppData
emptyAppData =
    AppData [] Nothing emptyProductInput


type Model
    = Loading
    | Loaded AppData
    | FailedLoading (Maybe AppData) Http.Error


getMaybeAppData : Model -> Maybe AppData
getMaybeAppData model =
    case model of
        Loading ->
            Nothing

        Loaded data ->
            Just data

        FailedLoading mbData _ ->
            mbData


setModelAppData model appData =
    case model of
        Loading ->
            model

        Loaded data ->
            Loaded appData

        FailedLoading mbData err ->
            FailedLoading (Just appData) err


setAppDataCurrentEditing currentEditing appData =
    { appData | currentEditing = currentEditing }


asCurrentEditingInAppData appData currentEditing =
    { appData | currentEditing = currentEditing }


setAppDataNewProductData newProductData appData =
    { appData | newProductData = newProductData }


asNewProductDataInAppData appData newProductData =
    { appData | newProductData = newProductData }


setAppDataProducts products appData =
    { appData | products = products }


asProductsInAppData appData products =
    { appData | products = products }


init : ( Model, Cmd Msg )
init =
    ( Loading
    , Http.get
        { url = "http://127.0.0.1:8088/products"
        , expect =
            Http.expectJson GotProducts (list productDecoder)
        }
    )



---- UPDATE ----


type Msg
    = NoOp
    | GotProducts (Result Http.Error (List Product))
    | ChangeNewProductName String
    | ChangeNewProductPrice String
    | ChangeNewProductStock String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        NoOp ->
            ( model, Cmd.none )

        GotProducts res ->
            case res of
                Ok products ->
                    ( case model of
                        Loading ->
                            Loaded
                                { emptyAppData
                                    | products = products
                                }

                        Loaded appData ->
                            Loaded <|
                                setAppDataProducts products appData

                        FailedLoading appData _ ->
                            let
                                data =
                                    case appData of
                                        Just val ->
                                            val

                                        Nothing ->
                                            emptyAppData
                            in
                            Loaded { data | products = products }
                    , Cmd.none
                    )

                Err err ->
                    ( FailedLoading Nothing err, Cmd.none )

        ChangeNewProductName val ->
            ( case getMaybeAppData model of
                Just data ->
                    let
                        newAppData =
                            val
                                |> asNameInProductInput data.newProductData
                                |> asNewProductDataInAppData data
                    in
                    setModelAppData model newAppData

                Nothing ->
                    model
            , Cmd.none
            )

        ChangeNewProductStock val ->
            ( case getMaybeAppData model of
                Just data ->
                    let
                        newAppData =
                            Maybe.withDefault data.newProductData.stock (String.toFloat val)
                                |> asStockInProductInput data.newProductData
                                |> asNewProductDataInAppData data
                    in
                    setModelAppData model newAppData

                Nothing ->
                    model
            , Cmd.none
            )

        ChangeNewProductPrice val ->
            ( case getMaybeAppData model of
                Just data ->
                    let
                        newAppData =
                            String.toInt val
                                |> asPriceInProductInput data.newProductData
                                |> asNewProductDataInAppData data
                    in
                    setModelAppData model newAppData

                Nothing ->
                    model
            , Cmd.none
            )



---- VIEW ----


view : Model -> Html Msg
view model =
    case model of
        Loading ->
            div []
                [ text "loading..." ]

        Loaded data ->
            div []
                [ renderNewProductForm data.newProductData
                , renderProducts data.products
                ]

        FailedLoading maybeData err ->
            case maybeData of
                Just data ->
                    div []
                        [ renderNewProductForm data.newProductData
                        , renderProducts data.products
                        ]

                Nothing ->
                    div []
                        [ text <| Debug.toString err ]


renderNewProductForm : ProductInput -> Html Msg
renderNewProductForm formData =
    div []
        [ text "Here you can create a new product"
        , br [] []
        , div []
            [ text "Name"
            , input
                [ Attributes.value formData.name
                , onInput ChangeNewProductName
                ]
                []
            ]
        , br [] []
        , div []
            [ text "Stock"
            , input
                [ Attributes.value <| String.fromFloat formData.stock
                , onInput ChangeNewProductStock
                ]
                []
            ]
        , br [] []
        , div []
            <| [ text "Price"
            , input
                [ Attributes.value <|
                    Maybe.withDefault "" <|
                        Maybe.map String.fromInt formData.price
                , onInput ChangeNewProductPrice
                ]
                []
            ]
            ++ if
                    case formData.price of
                        Just _ ->
                            False

                        Nothing ->
                            True
                then
                    [ br [] []
                    , text "You are making it free by the way."
                    ]

                else
                    []
               
        ]


renderProducts : List Product -> Html Msg
renderProducts products =
    case products of
        [] ->
            div [] [ text "No products there yet" ]

        _ ->
            div [] <|
                List.map renderProduct products


renderProduct : Product -> Html Msg
renderProduct product =
    div
        [ style "border" "2px solid red"
        , style "width" "30%"
        , style "margin-left" "35%"
        ]
        [ text <| "Name of this product: " ++ product.name
        , br [] []
        , text <| "There are " ++ String.fromFloat product.stock ++ " items in stock"
        , br [] []
        , text <|
            case product.price of
                Just actualPrice ->
                    "Each cost " ++ String.fromInt actualPrice ++ " <currency>"

                Nothing ->
                    "Actually, this product is free"
        , br [] []
        , text <| "User with id " ++ String.fromInt product.user_id ++ " owns this product"
        ]



---- PROGRAM ----


main : Program () Model Msg
main =
    Browser.element
        { view = view
        , init = \_ -> init
        , update = update
        , subscriptions = always Sub.none
        }
