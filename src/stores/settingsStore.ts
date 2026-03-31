import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

export type ScheduleFrequency = 'daily' | 'weekly' | 'workdays';

export interface SchedulerConfig {
  enabled: boolean;
  frequency: ScheduleFrequency;
  hour: number;
  minute: number;
  dayOfWeek?: number;
  repoIds: number[];
  promptId?: number | null;
}

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
    scheduler: {
      enabled: false,
      frequency: 'daily' as ScheduleFrequency,
      hour: 18,
      minute: 30,
      dayOfWeek: 5, // Friday
      repoIds: [] as number[],
      promptId: null as number | null,
    } as SchedulerConfig,
  }),
  actions: {
    async loadSettings() {
      try {
        const settings = await invoke<Record<string, string>>('get_settings');
        if ('ai.api_base_url' in settings) this.aiConfig.apiBaseUrl = settings['ai.api_base_url'];
        if ('ai.api_key' in settings) this.aiConfig.apiKey = settings['ai.api_key'];
        if ('ai.model_name' in settings) this.aiConfig.modelName = settings['ai.model_name'];
        if (settings['ai.temperature']) this.aiConfig.temperature = parseFloat(settings['ai.temperature']);
        if (settings['ai.max_tokens']) this.aiConfig.maxTokens = parseInt(settings['ai.max_tokens']);
        if ('git.user_name' in settings) this.gitUserName = settings['git.user_name'];
        if (settings['app.theme']) this.theme = settings['app.theme'] as 'light' | 'dark' | 'system';
      } catch (err) {
        console.error('Failed to load settings from backend:', err);
      }

      try {
        // 加载定时任务配置
        const schedulerConfig = await invoke<SchedulerConfig>('get_scheduler_config');
        this.scheduler = schedulerConfig;
      } catch (err) {
        console.error('Failed to load scheduler config from backend:', err);
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
