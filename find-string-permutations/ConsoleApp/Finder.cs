using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace ConsoleApp
{
    public class Finder : IFinder
    {
        public IEnumerable<Location> Find(string needle, string haystack)
        {
            var needles = GenerateNeedles(needle).Distinct().ToImmutableArray();

            var bales = (from b in GenerateBales(haystack, needle.Length)
                group b by b.bale into groupedBales
                select (bale: groupedBales.Key, positions: groupedBales.Select(gb => gb.position).ToArray()))
                .ToImmutableDictionary(b => b.bale, b => b.positions);
            
            foreach (var location in Find(needles, bales))
                yield return location;
        }

        private static IEnumerable<Location> Find(IReadOnlyList<string> needles, IDictionary<string, int[]> bales)
        {
            var needleCount = needles.Count;

            for (var needleIndex = 0; needleIndex < needleCount; needleIndex++)
            {
                var needle = needles[needleIndex];
                if (!bales.TryGetValue(needle, out var positions))
                {
                    continue;
                }

                var positionCount = positions.Length;
                for (var positionIndex = 0; positionIndex < positionCount; positionIndex++)
                {
                    yield return new(needle, positions[positionIndex]);
                }
            }
        }

        private static IEnumerable<string> GenerateNeedles(string needle)
        {
            var needleFragments = needle.ToCharArray();
            var needleFragmentCount = needleFragments.Length;

            for (var needleFragmentIndex = 0; needleFragmentIndex < needleFragmentCount; needleFragmentIndex++)
            {
                var needlePrefix = needleFragments[needleFragmentIndex];
                foreach (var generatedNeedle in GenerateNeedles(needlePrefix, new string(needleFragments.Take(needleFragmentIndex).ToArray()) + new string(needleFragments.Skip(needleFragmentIndex + 1).ToArray())))
                {
                    yield return generatedNeedle;
                }
            }
        }

        private static IEnumerable<string> GenerateNeedles(char needleHead, string needle)
        {
            var needleFragments = needle.ToCharArray();
            var needleFragmentCount = needleFragments.Length;

            if (needleFragmentCount == 0)
            {
                yield return needleHead.ToString();
            }

            for (var needleFragmentIndex = 0; needleFragmentIndex < needleFragmentCount; needleFragmentIndex++)
            {
                var needlePrefix = needleFragments[needleFragmentIndex];
                foreach (var generatedNeedle in GenerateNeedles(needlePrefix, new string(needleFragments.Take(needleFragmentIndex).ToArray()) + new string(needleFragments.Skip(needleFragmentIndex + 1).ToArray())))
                {
                    yield return needleHead + generatedNeedle;
                }
            }
        }

        private static IEnumerable<(string bale, int position)> GenerateBales(string haystack, int baleSize)
        {
            var baleCount = haystack.Length - baleSize + 1;
            for (var baleIndex = 0; baleIndex < baleCount; baleIndex++)
            {
                yield return (haystack.Substring(baleIndex, baleSize), baleIndex);
            }
        }
    }
}