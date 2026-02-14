import { Client } from "@elastic/elasticsearch";

export const elastic = new Client({
  node: "http://localhost:9200"
});

export const createIndex = async () => {
  const exists = await elastic.indices.exists({ index: "notes" });

  if (!exists) {
    await elastic.indices.create({
      index: "notes",
      body: {
        settings: {
          number_of_shards: 1,
          number_of_replicas: 0,
          analysis: {
            analyzer: {
              note_analyzer: {
                type: "custom",
                tokenizer: "standard",
                filter: ["lowercase", "asciifolding"]
              }
            }
          }
        },
        mappings: {
          properties: {
            title: { 
              type: "text",
              analyzer: "note_analyzer",
              fields: {
                keyword: { type: "keyword" }
              }
            },
            content: { 
              type: "text",
              analyzer: "note_analyzer"
            },
            tags: { 
              type: "text",
              fields: {
                keyword: { type: "keyword" }
              }
            },
            category: { 
              type: "text",
              fields: {
                keyword: { type: "keyword" }
              }
            },
            createdAt: { type: "date" }
          }
        }
      }
    });
    console.log("✅ Elasticsearch index created with custom analyzer");
  } else {
    console.log("ℹ️  Elasticsearch index already exists");
  }
};