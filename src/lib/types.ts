export type LocalModel = {
	name: string;
	modified_at: string;
	size: number;
};

export type GenerationResponse = {
	/** The name of the model used for the completion. */
	model: string;
	/** The creation time of the completion, in such format: `2023-08-04T08:52:19.385406455-07:00`. */
	created_at: String;
	/** The response of the completion. This can be the entire completion or only a token if the completion is streaming. */
	response: string;
	/** Whether the completion is done. If the completion is streaming, this will be false until the last response. */
	done: boolean;
	/**
	 * An encoding of the conversation used in this response,
	 * this can be sent in the next request to keep a conversational memory
	 */
	context?: GenerationContext | null;
	/** Time spent generating the response */
	total_duration?: number | null;
	/** Number of tokens in the prompt */
	prompt_eval_count?: number | null;
	/** Time spent in nanoseconds evaluating the prompt */
	prompt_eval_duration?: number | null;
	/** Number of tokens in the response */
	eval_count?: number | null;
	/** Time spent in nanoseconds generating the response */
	eval_duration?: number | null;
};

/**
 * An encoding of a conversation returned by Ollama after a completion request,
 * this can be sent in a new request to keep a conversational memory.
 */
export type GenerationContext = number[];
