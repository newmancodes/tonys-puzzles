using System;
using System.Diagnostics.CodeAnalysis;

namespace ConsoleApp
{
    [ExcludeFromCodeCoverage]
    public class Program
    {
        public static void Main(string[] args)
        {
            Console.WriteLine("What are you looking for?");
            var needle = Console.ReadLine();

            Console.WriteLine("Where are you looking?");
            var haystack = Console.ReadLine();

            Console.WriteLine($"Searching for permutations of '{needle}' within '{haystack}'.");

            IFinder finder = new Finder();
            foreach (var location in finder.Find(needle, haystack))
            {
                Console.WriteLine($"Found '{location.Needle}' at position '{location.Position}'.");
            }
        }
    }
}
