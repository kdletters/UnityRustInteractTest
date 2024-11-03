using System;
using System.Text;

namespace RustLib
{
    partial struct ByteBuffer
    {
        public unsafe Span<byte> AsSpan()
        {
            return new Span<byte>(ptr, length);
        }

        // public unsafe Span<T> AsSpan<T>()
        // {
        //     return MemoryMarshal.CreateSpan(ref Unsafe.AsRef<T>(ptr), length / Unsafe.SizeOf<T>());
        // }

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
            Lib.free_u8_string(name);
        }
    }
}