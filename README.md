# **DeScout**

Find high-quality airdrop opportunities, carefully and deeply analyzed with AI. Filter the best by selecting the analysis depth, choose your favorites, and track the required steps — all with the help of AI.

### **Technology**

> **Frontend**

* React + TypeScript

> **Server**

* Express + TypeScript for the app and website
* Python + Llama AI service for seamless AI interaction
* Rust for robust and fast web scraping, built to perform thousands of tasks seamlessly. It's divided into different microservices, each dedicated to a specific analysis depth level.
* MongoDB for flexible and easy data storage
* Redis for scraping task queues and fast data access
* Redis for deduplication tracking and query time reduction
* X (Twitter) API

### **Still Developing**

* Final steps for analysis depth level 2
* Dedicated merging microservice that will combine data before inserting it into the database, after collection from the same depth level
* AI analysis and summary for each depth level
* Additional analysis depth levels
* Identify smaller and faster LLM models
* Separate database for airdrop opportunities, which will store at least depth level 2 data
* Kafka for message queues and communication between microservices

### **Why Choose Such a Complex Model?**

* While harder to develop, it enables a more reliable and lower-cost product
* Using different databases may be complex, but ensures that if one fails, other processes — like the frontend, scraping, AI analysis, or data collection — will continue working seamlessly. The team can focus on fixing one isolated problem without affecting the rest of the system.

### **Next Steps**

* Add Discord scraping
* Add Telegram scraping
* Build an easy-to-use, addictive UI/UX for platform users
* Develop payment models for data access
