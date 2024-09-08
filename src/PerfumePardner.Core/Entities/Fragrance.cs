using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations.Schema;
using Newtonsoft.Json;
namespace perfume_pardner_api.Core.Entities
{
public class Fragrance
    {
        public int Id { get; set; }
        public string Title { get; set; }
        public string Designer { get; set; }
        public int Year { get; set; }
        public string Thumbnail { get; set; }
        [NotMapped]
        public Dictionary<string, List<string>> Url { get; set; }
        public string ObjectId { get; set; }

        // Add a parameterless constructor
        public Fragrance()
        {
            Url = new Dictionary<string, List<string>>();
        }

        public Fragrance(string title, string designer, Dictionary<string, List<string>> url, string objectId)
        {
            Title = title;
            Designer = designer;
            Url = url;
            ObjectId = objectId;
        }
        
    }
}