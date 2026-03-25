import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    aiConfig: {
      apiBaseUrl: 'https://api.openai.com/v1',
      apiKey: '',
      modelName: 'gpt-4o-mini',
      temperature: 0.5,
      maxTokens: 2048,
    },
    gitUserName: '',
    theme: 'light' as 'light' | 'dark' | 'system',
  }),
  actions: {
    async loadSettings() {
      try {
        const settings = await invoke<Record<string, string>>('get_settings');
        if (settings['ai.api_base_url']) this.aiConfig.apiBaseUrl = settings['ai.api_base_url'];
        if (settings['ai.api_key']) this.aiConfig.apiKey = settings['ai.api_key'];
        if (settings['ai.model_name']) this.aiConfig.modelName = settings['ai.model_name'];
        if (settings['ai.temperature']) this.aiConfig.temperature = parseFloat(settings['ai.temperature']);
        if (settings['ai.max_tokens']) this.aiConfig.maxTokens = parseInt(settings['ai.max_tokens']);
        if (settings['git.user_name']) this.gitUserName = settings['git.user_name'];
        if (settings['app.theme']) this.theme = settings['app.theme'] as 'light' | 'dark' | 'system';
      } catch (err) {
        console.error('Failed to load settings from backend:', err);
      }
    },
    async saveTheme(theme: 'light' | 'dark' | 'system') {
      this.theme = theme;
      try {
        await invoke('update_settings', { 
          settings: { 'app.theme': theme } 
        });
      } catch (err) {
        console.error('Failed to save theme setting:', err);
      }
    }
  }
});
