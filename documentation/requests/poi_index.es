curl -H "Content-Type: application/json" -XPOST "http://localhost:9200/_bulk" --data-binary "@bulk.json"

PUT poi_v1
{
	"mappings": {
		"properties": {
			"name": {
				"type": "text",
				"fields": {
					"raw": {
						"type": "keyword"
					},
					"suggest": {
						"type": "completion"
					}
				}
			},
			"description": {
				"type": "text"
			},
			"location": {
				"type": "geo_point"
			}
		}
	}
}