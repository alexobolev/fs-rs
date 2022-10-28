open System
open System.IO
open System.Reflection
open System.Threading
open Xunit.Runners

let lock obj func =
    Monitor.Enter obj
    try
        func ()
    finally
        Monitor.Exit obj

module Program =

    let cliLock = new Object()
    let testFinished = new ManualResetEvent(false)
    let mutable testResult = 0

    let onDiscoveryComplete (info : DiscoveryCompleteInfo) =
        let run = info.TestCasesToRun
        let discovered = info.TestCasesDiscovered
        lock cliLock (fun () ->
            printfn "Running %d / %d tests..." run discovered
        )
    
    let onExecutionComplete (info : ExecutionCompleteInfo) =
        let time = Math.Round(info.ExecutionTime, 3)
        let total = info.TotalTests
        let failed = info.TestsFailed
        let skipped = info.TestsSkipped
        lock cliLock (fun () ->
            printfn "Finished %d test(s) [%d failed, %d skipped] in %fs." total failed skipped time
        )
        testFinished.Set() |> ignore
    
    let onTestFailed (info : TestFailedInfo) =
        lock cliLock (fun () ->
            eprintfn "[FAILED ] %s -> %s" info.TestDisplayName info.ExceptionMessage
            if not <| isNull info.ExceptionStackTrace then
                eprintfn "%s" info.ExceptionStackTrace
        )
        testResult <- 1
    
    let onTestSkipped (info : TestSkippedInfo) =
        lock cliLock (fun () ->
            eprintfn "[SKIPPED] %s -> %s" info.TestDisplayName info.SkipReason
        )

    [<EntryPoint>]
    let main args =

        let getExeDirectory () =
            let assembly = Assembly.GetExecutingAssembly()
            assembly.Location |> Path.GetDirectoryName

        let rootInExeDirectory file =
            Path.GetFullPath(file, getExeDirectory())

        let (assemblyName, typeName) =
            match args.Length with
            | 1 -> (rootInExeDirectory args[0], null)
            | 2 -> (rootInExeDirectory args[0], args[1])
            | _ -> failwithf "Must have 1 or 2 arguments, had %A!" args
        
        use runner = AssemblyRunner.WithoutAppDomain assemblyName
        runner.OnDiscoveryComplete <- onDiscoveryComplete
        runner.OnExecutionComplete <- onExecutionComplete
        runner.OnTestFailed <- onTestFailed
        runner.OnTestSkipped <- onTestSkipped

        runner.Start typeName
        testFinished.WaitOne () |> ignore
        testFinished.Dispose () |> ignore

        testResult
