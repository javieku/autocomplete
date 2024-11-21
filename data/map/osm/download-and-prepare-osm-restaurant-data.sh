#!/bin/bash

wget -nc -O kanto-latest.osm.pbf https://download.geofabrik.de/asia/japan/kanto-latest.osm.pbf

output="./kanto-latest.osm.pbf"

if [ ! -e "$output" ]; then
  echo "Failed to download file."
  exit 1
fi


osmosis --read-pbf file="kanto-latest.osm.pbf" \
        --bounding-box left=139.750 bottom=35.660 right=139.770 top=35.670 \
        --tag-filter accept-nodes amenity=restaurant \
        --write-xml file="restaurants-tmp.xml"
      
output="./restaurants-tmp.xml"
if [ ! -e "$output" ]; then
  echo "Failed to process map data."
  exit 1
fi

yq -p=xml -o=json restaurants-tmp.xml >  restaurants-tmp.json

output="./restaurants-tmp.json"
if [ ! -e "$output" ]; then
  echo "Failed to convert xml map data to json."
  exit 1
fi

jq -r '
  .osm.node[] | 
  {
    id: ("jp:" + ."+@id"),
    name: (.tag[] | select(."+@k" == "name")."+@v"),
    timestamp: ."+@timestamp",
    location: {
      lat: ."+@lat",
      lon: ."+@lon"
    }
  }
' restaurants-tmp.json > restaurants-es.json

jq -c '{ index: { _index: "poi_v1", _id: .id } }, .' restaurants-es.json > bulk.json


output="./bulk.json"
if [ ! -e "$output" ]; then
  echo "Failed to convert to ES index format."
  exit 1
fi

echo "Data is ready to be ingested into elasticsearch."

# Clean cup temporary files
echo "Executing temporary files clean up"
find . -maxdepth 1 -type f -name '*tmp*' -exec rm -f {} \;

