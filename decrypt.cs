// https://dotnetfiddle.net/#

using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Security.Cryptography;
using System.Text;

public class Program
{
	public static void Main()
	{
		byte[] md5Buf = new byte[]{0xEE, 0x63, 0x75, 0x05, 0xBB};
		Console.Write("\nmd5Buf    ");
		foreach (byte i in md5Buf)
		{
			Console.Write("{0:X2}", i); // <- A byte is being passed.
		}

		Console.Write("\n");
		App app = new App();
		app.FileSize = 4388;
		byte[] decrypted = app.Decrypt(new byte[16]);
		Console.Write("\ndecrypted ");
		foreach (byte i in decrypted)
		{
			Console.Write("{0:X2}", i); // <- A byte is being passed.
		}
	}
}

public class App
{
	public byte[] encryptionStringA = new byte[16];
	public byte[] encryptionStringB = new byte[16];
	int eStringNo;
	int eStringPosA;
	int eStringPosB;
	int eNybbleSwap;
	public uint FileSize;
	public void GenerateELoadKeys(uint Arg1, uint Arg2, uint VSize)
	{
		GenerateKey(out encryptionStringA, Arg1);
		Console.Write("encryptionStringA: ");
		foreach (byte i in this.encryptionStringA)
		{
			Console.Write("{0:X2}", i); // <- A byte is being passed.
		}

		Console.WriteLine();
		GenerateKey(out encryptionStringB, Arg2);
		Console.Write("encryptionStringB: ");
		foreach (byte i in this.encryptionStringB)
		{
			Console.Write("{0:X2}", i); // <- A byte is being passed.
		}

		Console.WriteLine();
		eStringNo = (int)(VSize / 4) & 0x7F;
		Console.WriteLine("eStringNo ", eStringNo);
		eStringPosA = 0;
		eStringPosB = 8;
		eNybbleSwap = 0;
	}

	public void GenerateKey(out byte[] Buffer, uint Value)
	{
		string strbuf;
		byte[] md5Buf = new byte[16];
		strbuf = Value.ToString();
		md5Buf = CalculateMD5Hash(strbuf);
		Console.Write("md5: ");
		foreach (byte i in md5Buf)
		{
			Console.Write("{0:X2}", i); // <- A byte is being passed.
		}

		Console.WriteLine();
		Buffer = new byte[16];
		for (int y = 0; y < 16; y += 4)
		{
			// convert every 32-bit word to Little Endian
			Buffer[y + 3] = md5Buf[y + 0];		
			Buffer[y + 2] = md5Buf[y + 1];
			Buffer[y + 1] = md5Buf[y + 2];
			Buffer[y + 0] = md5Buf[y + 3];
		}

		return;
	}

	public byte[] CalculateMD5Hash(string input)	
	{
		MD5 md5 = System.Security.Cryptography.MD5.Create();
		byte[] inputBytes = System.Text.Encoding.ASCII.GetBytes(input);
		byte[] hash = md5.ComputeHash(inputBytes);
		return hash;
	}

	public byte[] Decrypt(byte[] data)
	{
		// Note: Since only XOr is used, this function does both,
		//       decryption and encryption.
		uint arg1 = FileSize;
		uint arg2 = ((this.FileSize >> 1) + 1);
		Console.WriteLine("FileSize {0:} {1:}", arg1, arg2);
		GenerateELoadKeys(arg1, arg2, arg1);
		const uint ENC_KEY_2 = 0x24924925;
		Console.WriteLine("ENC_KEY_2 0x{0:X}", ENC_KEY_2);
		const uint ENC_KEY_1 = 0xAAAAAAAB;
		int TempByt;
		int Key1;
		int Key2;
		int Temp1;
		int Temp2;
		byte[] ReturnData = new byte[data.Length];
		for (int i = 0; i < data.Length; i++)
		{
			TempByt = eStringNo ^ encryptionStringB[eStringPosB];
			TempByt ^= data[i];
			if (eNybbleSwap == 1) // swap nibbles: 0xAB <-> 0xBA
			{
				TempByt = ((TempByt << 4) + (TempByt >> 4)) & 0xFF;
			}

			TempByt ^= encryptionStringA[eStringPosA];
			ReturnData[i] = (byte)TempByt;
			eStringPosA++;
			eStringPosB++;
			if (eStringPosA <= 0x0F)
			{
				if (eStringPosB > 0x0C)
				{
					eStringPosB = 0;
					eNybbleSwap ^= 0x01;
				}
			}
			else if (eStringPosB <= 0x08)
			{
				eStringPosA = 0;
				eNybbleSwap ^= 0x01;
			}
			else
			{
				eStringNo += 2;
				eStringNo &= 0x7F;
				if (eNybbleSwap != 0)
				{
					Key1 = MulUnsignedHigh(ENC_KEY_1, eStringNo);
					Key2 = MulUnsignedHigh(ENC_KEY_2, eStringNo);
					eNybbleSwap = 0;
					Temp1 = Key2 + (eStringNo - Key2) / 2;
					Temp2 = Key1 / 8 * 3;
					eStringPosA = eStringNo - Temp1 / 4 * 7;
					eStringPosB = eStringNo - Temp2 * 4 + 2;
				}
				else
				{
					Key1 = MulUnsignedHigh(ENC_KEY_1, eStringNo);
					Key2 = MulUnsignedHigh(ENC_KEY_2, eStringNo);
					eNybbleSwap = 1;
					Temp1 = Key2 + (eStringNo - Key2) / 2;
					Temp2 = Key1 / 8 * 3;
					eStringPosB = eStringNo - Temp1 / 4 * 7;
					eStringPosA = eStringNo - Temp2 * 4 + 3;
				}
			}
		}

		return ReturnData;
	}

	public int MulUnsignedHigh(uint arg1, int arg2)
	{
		return (int)(((ulong)arg1 * (ulong)arg2) >> 32);
	}
}