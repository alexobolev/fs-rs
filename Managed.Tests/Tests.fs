module Tests

open System
open Xunit
open Program

[<Theory>]
[<InlineData("Jane", 60)>]
[<InlineData("Anna", 25)>]
[<InlineData("Maurice", 0)>]
let ``Constructs and returns the number`` (text: string) (number: int32) =
    use builder = new Interop.Builder(text, number)
    Assert.Equal(number, builder.Number)
