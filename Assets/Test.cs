using System;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using UnityEngine;

public class Test : MonoBehaviour
{
    private static Assembly assembly;
    
    public static void Load()
    {
        loadass
        assembly = Assembly.Load(File.ReadAllBytes(@"RustWrapper\RustWrapper\bin\Debug\net472\RustWrapper.dll"), File.ReadAllBytes(@"RustWrapper\RustWrapper\bin\Debug\net472\RustWrapper.pdb"));
    }

    public static void Add()
    {
        assembly.GetType("RustWrapper.Launcher").GetMethod("Main").Invoke(null, null);
    }
}