import express from "express";
import mongoose from "mongoose";
import cors from "cors";
import { connectMongo } from "./mongo.js";
import { elastic, createIndex } from "./elastic.js";

const app = express();
app.use(cors());
app.use(express.json());



// Connect databases
await connectMongo();
await createIndex();

// Note Model
const noteSchema = new mongoose.Schema({
  title: String,
  content: String,
  tags: [String],
  category: String
}, { timestamps: true });

const Note = mongoose.model("Note", noteSchema);

// ═══════════════════════════════════════════════════════════
// CREATE OPERATIONS
// ═══════════════════════════════════════════════════════════

// CREATE NOTE
app.post("/notes", async (req, res) => {
  try {
    const { title, content, tags = [], category = "general" } = req.body;

    const note = await Note.create({ title, content, tags, category });

    await elastic.index({
      index: "notes",
      id: note._id.toString(),
      document: {
        title,
        content,
        tags,
        category,
        createdAt: note.createdAt
      }
    });

    res.json({ success: true, note });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// BULK CREATE NOTES
app.post("/notes/bulk", async (req, res) => {
  try {
    const { notes } = req.body;

    if (!notes || !Array.isArray(notes)) {
      return res.status(400).json({ error: "Body must contain 'notes' array" });
    }

    const createdNotes = await Note.insertMany(notes);

    const operations = createdNotes.flatMap(note => [
      { index: { _index: "notes", _id: note._id.toString() } },
      {
        title: note.title,
        content: note.content,
        tags: note.tags || [],
        category: note.category || "general",
        createdAt: note.createdAt
      }
    ]);

    await elastic.bulk({ operations });

    res.json({ 
      success: true, 
      created: createdNotes.length,
      notes: createdNotes
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// SEED SAMPLE DATA
app.post("/notes/seed", async (req, res) => {
  try {
    const sampleNotes = [
      {
        title: "Introduction to JavaScript",
        content: "JavaScript is a versatile programming language used for web development. It runs in browsers and on servers with Node.js.",
        tags: ["javascript", "programming", "web"],
        category: "programming"
      },
      {
        title: "Python for Data Science",
        content: "Python is excellent for data science, machine learning, and AI. Libraries like NumPy, Pandas, and TensorFlow make it powerful.",
        tags: ["python", "data-science", "ml"],
        category: "programming"
      },
      {
        title: "MongoDB Basics",
        content: "MongoDB is a NoSQL database that stores data in JSON-like documents. It's flexible and scales horizontally.",
        tags: ["mongodb", "database", "nosql"],
        category: "database"
      },
      {
        title: "Elasticsearch Full-Text Search",
        content: "Elasticsearch is a distributed search engine built on Apache Lucene. Perfect for full-text search and analytics.",
        tags: ["elasticsearch", "search", "lucene"],
        category: "database"
      },
      {
        title: "React.js Frontend Development",
        content: "React is a JavaScript library for building user interfaces. It uses components and virtual DOM for efficient updates.",
        tags: ["react", "javascript", "frontend"],
        category: "programming"
      },
      {
        title: "Node.js Backend Development",
        content: "Node.js allows JavaScript to run on servers. Build REST APIs, microservices, and real-time applications.",
        tags: ["nodejs", "javascript", "backend"],
        category: "programming"
      },
      {
        title: "Docker Containerization",
        content: "Docker packages applications in containers. Ensures consistency across development, testing, and production environments.",
        tags: ["docker", "devops", "containers"],
        category: "devops"
      },
      {
        title: "Git Version Control",
        content: "Git tracks changes in code. Essential for collaboration, branching, merging, and maintaining code history.",
        tags: ["git", "version-control", "collaboration"],
        category: "tools"
      },
      {
        title: "GraphQL API Design",
        content: "GraphQL is a query language for APIs. Clients can request exactly what data they need, reducing over-fetching.",
        tags: ["graphql", "api", "backend"],
        category: "programming"
      },
      {
        title: "TypeScript Type Safety",
        content: "TypeScript adds static types to JavaScript. Catch errors at compile time and improve code maintainability.",
        tags: ["typescript", "javascript", "types"],
        category: "programming"
      },
      {
        title: "Redis Caching",
        content: "Redis is an in-memory data structure store. Use it for caching, session storage, and real-time features.",
        tags: ["redis", "caching", "database"],
        category: "database"
      },
      {
        title: "Kubernetes Orchestration",
        content: "Kubernetes manages containerized applications. Handles deployment, scaling, and management of container clusters.",
        tags: ["kubernetes", "devops", "containers"],
        category: "devops"
      },
      {
        title: "AWS Cloud Services",
        content: "Amazon Web Services provides cloud computing platforms. Services include EC2, S3, Lambda, and RDS for scalable applications.",
        tags: ["aws", "cloud", "infrastructure"],
        category: "cloud"
      },
      {
        title: "Machine Learning with TensorFlow",
        content: "TensorFlow is an open-source ML framework. Build neural networks for image recognition, NLP, and predictive analytics.",
        tags: ["tensorflow", "ml", "ai"],
        category: "ai"
      },
      {
        title: "Microservices Architecture",
        content: "Microservices break applications into small, independent services. Each service handles a specific business function.",
        tags: ["microservices", "architecture", "backend"],
        category: "architecture"
      }
    ];

    await Note.deleteMany({});
    
    try {
      await elastic.deleteByQuery({
        index: "notes",
        body: { query: { match_all: {} } }
      });
    } catch (e) {
      // Index might be empty
    }

    const createdNotes = await Note.insertMany(sampleNotes);

    const operations = createdNotes.flatMap(note => [
      { index: { _index: "notes", _id: note._id.toString() } },
      {
        title: note.title,
        content: note.content,
        tags: note.tags,
        category: note.category,
        createdAt: note.createdAt
      }
    ]);

    await elastic.bulk({ operations });

    // Refresh index for immediate search
    await elastic.indices.refresh({ index: "notes" });

    res.json({ 
      success: true, 
      message: "Database seeded with sample data",
      created: createdNotes.length
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// READ OPERATIONS
// ═══════════════════════════════════════════════════════════

// GET ALL NOTES
app.get("/notes", async (req, res) => {
  try {
    const notes = await Note.find().sort({ createdAt: -1 });
    res.json({ total: notes.length, notes });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// GET SINGLE NOTE
app.get("/notes/:id", async (req, res) => {
  try {
    const note = await Note.findById(req.params.id);
    if (!note) {
      return res.status(404).json({ error: "Note not found" });
    }
    res.json(note);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// 🔍 SEARCH OPERATIONS - FULL ELASTICSEARCH POWER!
// ═══════════════════════════════════════════════════════════

// 1️⃣ BASIC SEARCH (with fuzzy matching)
app.get("/search", async (req, res) => {
  try {
    const { q, page = 1, limit = 10 } = req.query;

    if (!q) {
      return res.status(400).json({ error: "Query parameter 'q' is required" });
    }

    const from = (page - 1) * limit;

    const result = await elastic.search({
      index: "notes",
      body: {
        from,
        size: parseInt(limit),
        query: {
          multi_match: {
            query: q,
            fields: ["title^3", "content^2", "tags"],  // title is 3x more important
            fuzziness: "AUTO",
            prefix_length: 1,
            operator: "or"
          }
        },
        highlight: {
          pre_tags: ["<mark>"],
          post_tags: ["</mark>"],
          fields: {
            title: {},
            content: { fragment_size: 150, number_of_fragments: 3 }
          }
        }
      }
    });

    const hits = result.hits.hits.map(hit => ({
      id: hit._id,
      score: hit._score,
      ...hit._source,
      highlights: hit.highlight || null
    }));

    res.json({ 
      query: q,
      page: parseInt(page),
      limit: parseInt(limit),
      total: result.hits.total.value,
      results: hits 
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// 2️⃣ ADVANCED SEARCH (with filters)
app.get("/search/advanced", async (req, res) => {
  try {
    const { 
      q,                    // search query
      category,             // filter by category
      tags,                 // filter by tags (comma-separated)
      from_date,            // filter from date
      to_date,              // filter to date
      sort = "relevance",   // sort by: relevance, date, title
      order = "desc",       // order: asc, desc
      page = 1,
      limit = 10
    } = req.query;

    const must = [];
    const filter = [];

    // Text search
    if (q) {
      must.push({
        multi_match: {
          query: q,
          fields: ["title^3", "content^2", "tags"],
          fuzziness: "AUTO"
        }
      });
    }

    // Category filter
    if (category) {
      filter.push({ term: { "category.keyword": category } });
    }

    // Tags filter
    if (tags) {
      const tagList = tags.split(",").map(t => t.trim());
      filter.push({ terms: { "tags.keyword": tagList } });
    }

    // Date range filter
    if (from_date || to_date) {
      const range = { createdAt: {} };
      if (from_date) range.createdAt.gte = from_date;
      if (to_date) range.createdAt.lte = to_date;
      filter.push({ range });
    }

    // Sort configuration
    let sortConfig;
    switch (sort) {
      case "date":
        sortConfig = [{ createdAt: order }];
        break;
      case "title":
        sortConfig = [{ "title.keyword": order }];
        break;
      default:
        sortConfig = q ? [{ _score: "desc" }] : [{ createdAt: "desc" }];
    }

    const result = await elastic.search({
      index: "notes",
      body: {
        from: (page - 1) * limit,
        size: parseInt(limit),
        query: {
          bool: {
            must: must.length > 0 ? must : [{ match_all: {} }],
            filter
          }
        },
        sort: sortConfig,
        highlight: {
          fields: { title: {}, content: {} }
        }
      }
    });

    const hits = result.hits.hits.map(hit => ({
      id: hit._id,
      score: hit._score,
      ...hit._source,
      highlights: hit.highlight || null
    }));

    res.json({
      query: q,
      filters: { category, tags, from_date, to_date },
      sort,
      order,
      page: parseInt(page),
      limit: parseInt(limit),
      total: result.hits.total.value,
      results: hits
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// 3️⃣ PHRASE SEARCH (exact phrase matching)
app.get("/search/phrase", async (req, res) => {
  try {
    const { q, slop = 0 } = req.query;

    if (!q) {
      return res.status(400).json({ error: "Query parameter 'q' is required" });
    }

    const result = await elastic.search({
      index: "notes",
      body: {
        query: {
          multi_match: {
            query: q,
            fields: ["title", "content"],
            type: "phrase",
            slop: parseInt(slop)  // allows words between phrase words
          }
        },
        highlight: {
          fields: { title: {}, content: {} }
        }
      }
    });

    const hits = result.hits.hits.map(hit => ({
      id: hit._id,
      score: hit._score,
      ...hit._source,
      highlights: hit.highlight || null
    }));

    res.json({ 
      query: q,
      type: "phrase",
      slop: parseInt(slop),
      total: result.hits.total.value,
      results: hits 
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// 4️⃣ WILDCARD SEARCH (pattern matching)

app.get("/search/wildcard", async (req, res) => {
  try {
    const { q } = req.query;

    if (!q) {
      return res.status(400).json({ error: "Query parameter 'q' is required" });
    }

    const result = await elastic.search({
      index: "notes",
      body: {
        query: {
          wildcard: {
            content: {
              value: `*${q.toLowerCase()}*`,
              case_insensitive: true
            }
          }
        },
        highlight: {
          fields: { content: {} }
        }
      }
    });

    const hits = result.hits.hits.map(hit => ({
      id: hit._id,
      score: hit._score,
      ...hit._source,
      highlights: hit.highlight || null
    }));

    res.json({ 
      query: q,
      type: "wildcard",
      total: result.hits.total.value,
      results: hits 
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// 5️⃣ AUTOCOMPLETE / SUGGESTIONS
app.get("/search/suggest", async (req, res) => {
  try {
    const { q } = req.query;

    if (!q) {
      return res.status(400).json({ error: "Query parameter 'q' is required" });
    }

    const result = await elastic.search({
      index: "notes",
      body: {
        size: 5,
        query: {
          bool: {
            should: [
              {
                match_phrase_prefix: {
                  title: {
                    query: q,
                    max_expansions: 10
                  }
                }
              },
              {
                match_phrase_prefix: {
                  content: {
                    query: q,
                    max_expansions: 10
                  }
                }
              }
            ]
          }
        },
        _source: ["title"]
      }
    });

    const suggestions = result.hits.hits.map(hit => ({
      id: hit._id,
      title: hit._source.title,
      score: hit._score
    }));

    res.json({ 
      query: q,
      suggestions
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// 6️⃣ MORE LIKE THIS (similar notes)
app.get("/search/similar/:id", async (req, res) => {
  try {
    const { id } = req.params;

    const result = await elastic.search({
      index: "notes",
      body: {
        query: {
          more_like_this: {
            fields: ["title", "content", "tags"],
            like: [{ _index: "notes", _id: id }],
            min_term_freq: 1,
            min_doc_freq: 1,
            max_query_terms: 25
          }
        }
      }
    });

    const hits = result.hits.hits.map(hit => ({
      id: hit._id,
      score: hit._score,
      ...hit._source
    }));

    res.json({ 
      similarTo: id,
      total: result.hits.total.value,
      results: hits 
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// 📊 AGGREGATIONS (Analytics)
// ═══════════════════════════════════════════════════════════

// CATEGORY STATS
app.get("/stats/categories", async (req, res) => {
  try {
    const result = await elastic.search({
      index: "notes",
      body: {
        size: 0,
        aggs: {
          categories: {
            terms: { field: "category.keyword" }
          }
        }
      }
    });

    res.json({
      total: result.hits.total.value,
      categories: result.aggregations.categories.buckets
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// TAG CLOUD
app.get("/stats/tags", async (req, res) => {
  try {
    const result = await elastic.search({
      index: "notes",
      body: {
        size: 0,
        aggs: {
          tags: {
            terms: { 
              field: "tags.keyword",
              size: 50
            }
          }
        }
      }
    });

    res.json({
      total: result.hits.total.value,
      tags: result.aggregations.tags.buckets
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// NOTES BY DATE
app.get("/stats/timeline", async (req, res) => {
  try {
    const result = await elastic.search({
      index: "notes",
      body: {
        size: 0,
        aggs: {
          notes_over_time: {
            date_histogram: {
              field: "createdAt",
              calendar_interval: "day"
            }
          }
        }
      }
    });

    res.json({
      timeline: result.aggregations.notes_over_time.buckets
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// UPDATE OPERATIONS
// ═══════════════════════════════════════════════════════════

// UPDATE NOTE
app.put("/notes/:id", async (req, res) => {
  try {
    const { id } = req.params;
    const { title, content, tags, category } = req.body;

    const note = await Note.findByIdAndUpdate(
      id,
      { title, content, tags, category },
      { new: true }
    );

    if (!note) {
      return res.status(404).json({ error: "Note not found" });
    }

    await elastic.update({
      index: "notes",
      id,
      body: {
        doc: {
          title,
          content,
          tags,
          category
        }
      }
    });

    res.json({ success: true, note });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// DELETE OPERATIONS
// ═══════════════════════════════════════════════════════════

// DELETE SINGLE NOTE
app.delete("/notes/:id", async (req, res) => {
  try {
    const { id } = req.params;

    await Note.findByIdAndDelete(id);
    await elastic.delete({ index: "notes", id });

    res.json({ success: true, message: "Deleted successfully" });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// DELETE ALL NOTES
app.delete("/notes", async (req, res) => {
  try {
    const result = await Note.deleteMany({});
    
    await elastic.deleteByQuery({
      index: "notes",
      body: { query: { match_all: {} } }
    });

    res.json({ 
      success: true, 
      message: "All notes deleted",
      deletedCount: result.deletedCount 
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// UTILITY ENDPOINTS
// ═══════════════════════════════════════════════════════════

// REINDEX ALL DATA (MongoDB → Elasticsearch)
app.post("/reindex", async (req, res) => {
  try {
    // Delete existing index
    try {
      await elastic.indices.delete({ index: "notes" });
    } catch (e) {}

    // Recreate index
    await createIndex();

    // Get all notes from MongoDB
    const notes = await Note.find();

    if (notes.length === 0) {
      return res.json({ success: true, message: "No notes to reindex" });
    }

    // Bulk index
    const operations = notes.flatMap(note => [
      { index: { _index: "notes", _id: note._id.toString() } },
      {
        title: note.title,
        content: note.content,
        tags: note.tags || [],
        category: note.category || "general",
        createdAt: note.createdAt
      }
    ]);

    await elastic.bulk({ operations });
    await elastic.indices.refresh({ index: "notes" });

    res.json({ 
      success: true, 
      message: "Reindex complete",
      indexed: notes.length 
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// HEALTH CHECK
app.get("/health", async (req, res) => {
  try {
    const mongoStatus = mongoose.connection.readyState === 1;
    const esHealth = await elastic.cluster.health();

    res.json({
      status: "ok",
      mongodb: mongoStatus ? "connected" : "disconnected",
      elasticsearch: {
        status: esHealth.status,
        cluster: esHealth.cluster_name,
        nodes: esHealth.number_of_nodes
      }
    });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

// ═══════════════════════════════════════════════════════════
// START SERVER
// ═══════════════════════════════════════════════════════════

app.listen(3000, () => {
  console.log("\n🚀 Server running on http://localhost:3000");
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("📝 CRUD ENDPOINTS:");
  console.log("═══════════════════════════════════════════════════════════");
  console.log("POST   /notes              - Create single note");
  console.log("POST   /notes/bulk         - Create multiple notes");
  console.log("POST   /notes/seed         - Seed sample data (15 notes)");
  console.log("GET    /notes              - Get all notes");
  console.log("GET    /notes/:id          - Get single note");
  console.log("PUT    /notes/:id          - Update note");
  console.log("DELETE /notes/:id          - Delete single note");
  console.log("DELETE /notes              - Delete all notes");
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("🔍 SEARCH ENDPOINTS:");
  console.log("═══════════════════════════════════════════════════════════");
  console.log("GET    /search?q=term      - Basic fuzzy search");
  console.log("GET    /search/advanced    - Advanced search with filters");
  console.log("GET    /search/phrase?q=   - Exact phrase search");
  console.log("GET    /search/wildcard?q= - Wildcard pattern search");
  console.log("GET    /search/suggest?q=  - Autocomplete suggestions");
  console.log("GET    /search/similar/:id - Find similar notes");
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("📊 ANALYTICS ENDPOINTS:");
  console.log("═══════════════════════════════════════════════════════════");
  console.log("GET    /stats/categories   - Notes count by category");
  console.log("GET    /stats/tags         - Tag cloud");
  console.log("GET    /stats/timeline     - Notes over time");
  console.log("\n═══════════════════════════════════════════════════════════");
  console.log("🔧 UTILITY ENDPOINTS:");
  console.log("═══════════════════════════════════════════════════════════");
  console.log("POST   /reindex            - Rebuild Elasticsearch index");
  console.log("GET    /health             - Health check");
  console.log("\n═══════════════════════════════════════════════════════════\n");
});