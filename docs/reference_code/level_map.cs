using System;

public class Program
{
	public static void Main()
	{
		Console.WriteLine("{0:X}", 0x00 << 8);
		Console.WriteLine("{0:X}", 0x01 << 8);
		Console.WriteLine("{0:X}", 0x33 << 8);
		Console.WriteLine("{0:}", (0x00 << 8) + 0x67);
		
		byte[] mappingEntry = new byte[3];
		mappingEntry[0] = 0x04;
		mappingEntry[1] = 0x38;
		mappingEntry[2] = 0x33;
		
		mappingEntry[0] = (byte)(mappingEntry[0] - (mappingEntry[0] >> 6 << 6));
		byte VisualPlane = (byte)(mappingEntry[0] >> 4);
		mappingEntry[0] = (byte)(mappingEntry[0] - (mappingEntry[0] >> 4 << 4));
		byte Direction = (byte)(mappingEntry[0] >> 2);
		mappingEntry[0] = (byte)(mappingEntry[0] - (mappingEntry[0] >> 2 << 2));
		ushort Tile16x16 = (ushort)((mappingEntry[0] << 8) + mappingEntry[1]);
		byte CollisionFlag0 = (byte)(mappingEntry[2] >> 4);
		byte CollisionFlag1 = (byte)(mappingEntry[2] - (mappingEntry[2] >> 4 << 4));
		
		Console.WriteLine("0x{0:X} 0x{1:X} 0x{2:X}", mappingEntry[0], mappingEntry[1], mappingEntry[2]);
		Console.WriteLine("VisualPlane 0x{0:X} Direction 0x{1:X} Tile16x16 {2:}", VisualPlane, Direction, Tile16x16);
		Console.WriteLine("0x{0:X}{1:X}", CollisionFlag0, CollisionFlag1);
	}
}