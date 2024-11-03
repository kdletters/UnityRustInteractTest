using System;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using UnityEditor;
using UnityEngine;

public static class SetUp
{
#if UNITY_EDITOR_WIN
    private const string RustDLLPath = "rust-unity/target/release/rust_unity.dll";
    private static IntPtr hModule;
#endif
    
    private static Assembly assembly;

    [InitializeOnLoadMethod]
    private static void LoadDLL()
    {
        EditorApplication.playModeStateChanged += change =>
        {
            switch (change)
            {
                case PlayModeStateChange.EnteredPlayMode:
#if UNITY_EDITOR_WIN
                    var path = new FileInfo(RustDLLPath).FullName;
                    hModule = LoadLibrary(path);
#endif
                    // assembly = Assembly.Load(File.ReadAllBytes(@"RustWrapper\RustWrapper\bin\Debug\netstandard2.1\RustWrapper.dll"), File.ReadAllBytes(@"RustWrapper\RustWrapper\bin\Debug\netstandard2.1\RustWrapper.pdb"));
                    break;
                case PlayModeStateChange.ExitingPlayMode:
                    UnloadDLL();
                    break;
                case PlayModeStateChange.EnteredEditMode:
                case PlayModeStateChange.ExitingEditMode:
                    break;
                default:
                    throw new ArgumentOutOfRangeException(nameof(change), change, null);
            }
        };
    }

    private static void UnloadDLL()
    {
#if UNITY_EDITOR_WIN
        FreeLibrary(hModule);
        hModule = IntPtr.Zero;
#endif
    }

#if UNITY_EDITOR_WIN
    [DllImport("kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    private static extern IntPtr LoadLibrary(string lpFileName);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool FreeLibrary(IntPtr hModule);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr GetLastError();
#endif
}