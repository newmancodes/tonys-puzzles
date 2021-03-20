using System;
using System.Collections.Generic;
using System.Linq;

namespace ConsoleApp
{
    public class Program
    {
        public static void Main(string[] args)
        {
            var range = Enumerable.Range(1, 1000);
            var cubes = range.Select(v => (value: v, cube: Math.Pow(v, 3))).ToList();
            Console.WriteLine("...Generated cubes");
            var twoPartSums = new List<((double value, double cube) left, (double value, double cube) right, double sum, (int value, double cube)[] remaining)>();

            for (int cubeIndex = 0; cubeIndex < cubes.Count - 1; cubeIndex++)
            {
                var cube = cubes[cubeIndex];

                for (int additionPartnerCubeIndex = cubeIndex + 1;
                    additionPartnerCubeIndex < cubes.Count;
                    additionPartnerCubeIndex++)
                {
                    var additionPartnerCube = cubes[additionPartnerCubeIndex];
                    twoPartSums.Add(new (cube, additionPartnerCube, cube.cube + additionPartnerCube.cube, cubes.Where(c => c != cube && c != additionPartnerCube).ToArray()));
                }
            }
            Console.WriteLine("...Generated sums");

            for (int twoPartSumIndex = 0; twoPartSumIndex < twoPartSums.Count; twoPartSumIndex++)
            {
                var twoPartSum = twoPartSums[twoPartSumIndex];
                for (int remainingIndex = 0; remainingIndex < twoPartSum.remaining.Length; remainingIndex++)
                {
                    var remaining = twoPartSum.remaining[remainingIndex];
                    var subtracted = twoPartSum.sum - remaining.cube;
                    var ds =twoPartSum.remaining.Where(r => r.cube == subtracted && r.value != remaining.value).ToArray(); 
                    
                    for (int dIndex = 0; dIndex < ds.Length; dIndex++)
                    {
                        var d = ds[dIndex];
                        Console.WriteLine($"a: {twoPartSum.left.value}, b: {twoPartSum.right.value}, c: {remaining.value}, d: {d.value}");
                        Console.WriteLine($"{twoPartSum.left.cube} + {twoPartSum.right.cube} = {remaining.cube} + {d.cube}");
                        Console.WriteLine();
                    }
                }
            }
        }
    }
}
