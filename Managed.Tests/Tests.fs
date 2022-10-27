module Tests

open Xunit
open Program

[<Theory>]
[<InlineData("Jane", 60)>]
[<InlineData("Anna", 25)>]
[<InlineData("Maurice", 0)>]
let ``Constructs and can get values`` (name: string) (number: int32) =
    use builder = new Interop.Builder(name, number)
    Assert.Equal(number, builder.Number)
    Assert.Equal(name, builder.Name)
