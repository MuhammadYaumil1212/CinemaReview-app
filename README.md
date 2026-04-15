# 🎬 CinemaReview

## 📖 Project Description
**CinemaReview** is a **Soroban-based (Stellar Network)** smart contract that serves as a decentralized backend for a movie review community platform.

This contract enables fast and frictionless public interaction. Through this application, anyone can write movie opinions, use a pseudonym (username), and build community consensus through an upvote system. All discussion data is stored immutably within the blockchain's persistent storage.

Its lightweight architecture, focused on text data, makes it an ideal Web3 data layer for integration with modern application interfaces. It cleanly separates blockchain logic from visual metadata (such as images from Web2 APIs) in a structured and efficient manner.

---

## ✨ Key Features

* **📝 Open Review Catalog (`add_review`)**
  Anyone can freely add a new review for a movie. Users simply submit the movie ID, username, and review content. This data is automatically assigned a unique random ID (`prng().gen()`) and committed to the blockchain state.

* **🔍 Specific Discussion Filter (`get_movie_reviews`)**
  On-chain search logic that filters the review database and returns only the list of comments corresponding to the requested `movie_id`. This function is highly efficient for rendering specific discussions on the movie detail page of the client interface.

* **👍 Community Reputation System (`upvote_review`)**
  Utilizes a social interaction system where users can support specific reviews via upvotes. This creates organic visibility and comment ranking based on community assessment.

* **🗑️ Moderation & Reset Facility (`delete_review`)**
  A state management feature that allows the deletion of reviews based on their ID. This is essential for the development and sandbox testing phases, ensuring the contract storage can be cleared of dummy data.

---

## 🔑 Smart Contract ID
`CD7YN5MBD3B7FH46V33G67VCSRKCPSTDJHNABJEN3O22ZRKXVT3Y3DUO`

---

## Screenshoot

<img width="1470" height="799" alt="image" src="https://github.com/user-attachments/assets/35c3db85-ee0c-4d01-8a29-ed1a59bf6239" />
