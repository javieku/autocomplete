POST poi_v1/_search
{
    "size": 1000,
    "query": {
        "match_all": {}
    }
}

POST poi_v1/_search 
{
    "suggest": {
        "poi-suggestions": {
            "prefix": "寿司 ",
            // 寿司
            "completion": {
                "field": "name.suggest",
                "skip_duplicates": true
            }
        }
    }
}
