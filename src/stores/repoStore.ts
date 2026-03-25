import { defineStore } from 'pinia';

export interface Repository {
  id: number;
  name: string;
  path: string;
  isActive: boolean;
  createdAt: string;
}

export const useRepoStore = defineStore('repo', {
  state: () => ({
    repositories: [] as Repository[],
    selectedRepoIds: [] as number[],
    loading: false,
  }),
  actions: {
    setRepositories(repos: Repository[]) {
      this.repositories = repos;
    },
    setLoading(loading: boolean) {
      this.loading = loading;
    }
  }
});
