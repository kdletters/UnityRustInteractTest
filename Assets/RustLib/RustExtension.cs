using System;
using System.Runtime.InteropServices;
using System.Text;
using Unity.Collections.LowLevel.Unsafe;

namespace RustLib
{
    partial struct ByteBuffer
    {
        public unsafe Span<byte> AsSpan()
        {
            return new Span<byte>(ptr, length);
        }

        public unsafe Span<T> AsSpan<T>() where T : struct
        {
            return MemoryMarshal.CreateSpan(ref UnsafeUtility.AsRef<T>(ptr), length / UnsafeUtility.SizeOf<T>());
        }

        public override string ToString()
        {
            return Encoding.UTF8.GetString(AsSpan());
        }
    }

    partial struct Obj
    {
        public unsafe void Dispose()
        {
            Console.WriteLine("Disposing");
            Lib.free_byte_buffer(name);
        }
    }
}