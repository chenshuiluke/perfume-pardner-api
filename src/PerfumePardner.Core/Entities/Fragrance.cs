using System;
using System.Collections.Generic;
using Newtonsoft.Json;
namespace perfume_pardner_api;

public class Fragrance
{
    [JsonProperty("title")]
    public string Title { get; set; }

    [JsonProperty("designer")]
    public string Designer { get; set; }

    [JsonProperty("year")]
    public int Year { get; set; }

    [JsonProperty("thumbnail")]
    public string Thumbnail { get; set; }

    [JsonProperty("url")]
    public Dictionary<string, List<string>> Url { get; set; }

    [JsonProperty("objectID")]
    public string ObjectId { get; set; }

    public Fragrance(string naslov, string dizajner, int godina, string thumbnail, Dictionary<string, List<string>> url, string objectID)
    {
        Title = naslov;
        Designer = dizajner;
        Year = godina;
        Thumbnail = thumbnail;
        Url = url;
        ObjectId = objectID;
    }
}