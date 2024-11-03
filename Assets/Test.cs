using System;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.UI;

public class Test : MonoBehaviour
{
    public static void Add(Text text)
    {
        text.text = RustLib.Lib.add(1, 10).ToString();
        // AppDomain.CurrentDomain.GetAssemblies().First(x => x.GetName().Name == "RustWrapper").GetType("RustWrapper.Launcher").GetMethod("Main").Invoke(null, null);
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