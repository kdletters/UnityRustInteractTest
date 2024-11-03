using System;
using System.Text;

namespace RustWrapper
{
    public static class Launcher
    {
        public static unsafe void Main()
        {
            var rustStr = RustLib.Lib.alloc_u8_string();
            Console.WriteLine(rustStr->ToString());
            // UnityEngine.Debug.Log(rustObj);
            var rustObj = RustLib.Lib.get();
            Console.WriteLine(rustObj->name->ToString());
            rustObj->Dispose();
        }
    }
}