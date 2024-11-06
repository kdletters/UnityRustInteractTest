using System;
using System.Runtime.InteropServices;
using static RustLib.Lib;
using UnityEngine;
using UnityEngine.UI;

public class Test : MonoBehaviour
{
    public static unsafe void Add(Text text)
    {
        var obj = get();
        print(obj->name->ToString());
        free_game((byte*)obj);
        
        text.text = add(1, 10).ToString();

        var game = create_game(10, 10, 10);
        var block = get_block(game, 1, 1);
        print(JsonUtility.ToJson(*block));
        open_block(game, 1, 1);
        print(JsonUtility.ToJson(*block));
        free_game(game);
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