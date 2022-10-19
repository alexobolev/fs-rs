open System
open System.Runtime.InteropServices

module internal InteropInternal =

    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern IntPtr internal BuilderCreate (string name, int32 number)
    
    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern void internal BuilderFree (IntPtr pointer)
    
    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern int32 internal BuilderGetNumber (IntPtr pointer)

    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern void internal BuilderSetNumber (IntPtr pointer, int32 value)

module public Interop =

    type Builder(name: string, number: int) =

        // call rust constructor or managed initialization
        let handle : IntPtr = InteropInternal.BuilderCreate (name, number)

        // field to store destruction status
        let mutable disposed : bool = false

        let cleanup (disposing : bool) =
            if not disposed then
                disposed <- true
                // if disposing then
                //     ()
                InteropInternal.BuilderFree (handle)

        interface IDisposable with
            member self.Dispose() =
                cleanup (true)
                GC.SuppressFinalize(self)

        override self.Finalize() =
            cleanup (false)

        member self.Number
            with get () : int32 = InteropInternal.BuilderGetNumber (handle)
            and set (value : int32) = InteropInternal.BuilderSetNumber (handle, value)

    

[<EntryPoint>]
let main args =

    printfn "Hello from F#!"

    for index, arg in Seq.indexed args do
        printfn "Arg #%d => %s." index arg

    use foo = new Interop.Builder ("John", 15)
    printfn "Builder was initialized with value %d" foo.Number

    0
