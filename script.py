from transformers import pipeline

# Initialize the generator with more explicit parameters
generator = pipeline(
    'text-generation',
    model='EleutherAI/gpt-neo-2.7B',
    device=0  # Use GPU if available, -1 for CPU
)

# Generate text with more controlled parameters
response = generator(
    "EleutherAI has",
    do_sample=True,
    min_length=50,
    max_length=100,  # Add maximum length constraint
    temperature=0.7,  # Control randomness (0.0-1.0)
    num_return_sequences=1,  # Number of different completions
    pad_token_id=generator.tokenizer.eos_token_id
)

print(response[0]['generated_text']) 