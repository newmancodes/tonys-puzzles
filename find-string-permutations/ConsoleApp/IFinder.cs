using System.Collections.Generic;

namespace ConsoleApp
{
    public interface IFinder
    {
        IEnumerable<Location> Find(string needle, string haystack);
    }
}