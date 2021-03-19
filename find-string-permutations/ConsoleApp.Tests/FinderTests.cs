using System.Linq;
using FluentAssertions;
using Xunit;

namespace ConsoleApp.Tests
{
    public class FinderTests
    {
        [Fact]
        public void Needle_And_Haystack_Are_Identical_So_Location_Is_Start()
        {
            // Arrange
            var sut = new Finder();
            
            // Act
            var locations = sut.Find("a", "a");
            
            // Assert
            var expected = new Location[] { new ("a", 0) };
            locations.Should().BeEquivalentTo(expected);
        }

        [Fact]
        public void Needle_Found_Within_Haystack_Once()
        {
            // Arrange
            var sut = new Finder();
            
            // Act
            var locations = sut.Find("b", "ab");
            
            // Assert
            var expected = new Location[] { new ("b", 1) };
            locations.Should().BeEquivalentTo(expected);
        }

        [Fact]
        public void Needle_Found_Within_Haystack_Twice()
        {
            // Arrange
            var sut = new Finder();
            
            // Act
            var locations = sut.Find("b", "abb");
            
            // Assert
            var expected = new Location[] { new ("b", 1), new("b", 2) };
            locations.Should().BeEquivalentTo(expected);
        }

        [Fact]
        public void Needle_Found_Within_Haystack_Twice_If_Permuted()
        {
            // Arrange
            var sut = new Finder();
            
            // Act
            var locations = sut.Find("ab", "abba").ToArray();
            
            // Assert
            var expected = new Location[] { new ("ab", 0), new("ba", 2) };
            locations.Should().BeEquivalentTo(expected);
        }
    }
}
