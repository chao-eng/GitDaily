import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

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
    async loadRepos() {
      this.loading = true;
      try {
        const repos = await invoke<Repository[]>('list_repositories');
        this.repositories = repos.map(r => ({
          id: r.id,
          name: r.name,
          path: r.path,
          isActive: (r as any).enabled ?? true, // 兼容 Rust 端的 enabled 字段
          createdAt: (r as any).created_at || ''
        }));
      } catch (err) {
        console.error('Failed to load repositories:', err);
      } finally {
        this.loading = false;
      }
    },
    setRepositories(repos: Repository[]) {
      this.repositories = repos;
    },
    setLoading(loading: boolean) {
      this.loading = loading;
    }
  }
});
