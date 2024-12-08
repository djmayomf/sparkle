pub struct MemorySystem {
    short_term: ShortTermMemory,
    long_term: LongTermMemory,
    working_memory: WorkingMemory,
    memory_indexer: MemoryIndexer,
}

impl MemorySystem {
    pub async fn process_experience(&mut self, experience: Experience) -> Result<()> {
        // Add to short-term memory
        self.short_term.add_experience(experience.clone()).await?;
        
        // Process for long-term storage
        if self.should_store_long_term(&experience) {
            self.long_term.store_memory(experience).await?;
        }
        
        // Update working memory
        self.working_memory.update_context(experience).await?;
        
        // Index new memories
        self.memory_indexer.index_new_memories().await?;
        
        Ok(())
    }
} 