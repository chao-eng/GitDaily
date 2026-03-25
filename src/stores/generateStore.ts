import { defineStore } from 'pinia';

export interface CommitRecord {
  hash: string;
  shortHash: string;
  authorName: string;
  authorEmail: string;
  timestamp: number;
  datetime: string;
  message: string;
  repoName: string;
  isMerge: boolean;
}

export const useGenerateStore = defineStore('generate', {
  state: () => ({
    commits: [] as CommitRecord[],
    dateRange: ['', ''] as [string, string],
    selectedPromptId: null as number | null,
    generatedContent: '',
    isGenerating: false,
    isStreaming: false,
  }),
});
