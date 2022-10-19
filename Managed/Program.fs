open System
open System.Diagnostics
open System.Runtime.InteropServices
open System.Threading

module internal InteropInternal =

    (* Constructor/destructor. *)

    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern IntPtr internal BuilderCreate (string name, int32 number)
    
    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern void internal BuilderFree (IntPtr pointer)

    (* Number getter/setter. *)
    
    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern int32 internal BuilderGetNumber (IntPtr pointer)

    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern void internal BuilderSetNumber (IntPtr pointer, int32 value)

    (* Name getter/setter. *)
    
    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern string internal BuilderGetName (IntPtr pointer)

    [<DllImport("native", CallingConvention = CallingConvention.Cdecl)>]
    extern void internal BuilderSetName (IntPtr pointer, string value)

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
        
        member self.Name
            with get () : string = InteropInternal.BuilderGetName (handle)
            and set (value : string) = InteropInternal.BuilderSetName (handle, value)

    

[<EntryPoint>]
let main args =

    printfn "Process ID = %d." (Process.GetCurrentProcess().Id)

    for index, arg in Seq.indexed args do
        printfn "Process arg #%d => %s." (index + 1) arg

    if Seq.contains "--waitforit" args then
        eprintfn "Waiting for user signal..."
        Console.ReadKey () |> ignore
        eprintfn "Signaled! Moving on..."

    use foo = new Interop.Builder ("John", 15)
    printfn "Builder was initialized with values %d and %s" foo.Number foo.Name

    0
