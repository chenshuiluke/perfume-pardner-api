namespace perfume_pardner_api.Models 
{
    public class FragranceDto
    {
        public string Title { get; }
        public string Designer { get; }
        public int Year { get; set; }
        public string? Thumbnail { get; set; }
        public Dictionary<string, List<string>> Url { get; }
        public string ObjectId { get; }

        public FragranceDto(string title, string designer, Dictionary<string, List<string>> url, string objectId)
        {
            Title = title;
            Designer = designer;
            Url = url;
            ObjectId = objectId;
        }
    }
}