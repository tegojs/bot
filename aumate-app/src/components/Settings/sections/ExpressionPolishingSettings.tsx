import { PasswordInput, SettingRow } from "@/components/ui";
import { useSettingsStore } from "@/stores/settingsStore";

export function ExpressionPolishingSettings() {
  const { settings, updateExpressionPolishing } = useSettingsStore();
  const { expression_polishing } = settings;

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-white">Expression Polishing</h2>
      <p className="text-sm text-gray-400">
        Configure the AI-powered expression polishing feature. Press Tab in the
        command palette to switch to polish mode.
      </p>

      <div className="space-y-4">
        {/* API URL */}
        <SettingRow
          label="API URL"
          description="OpenAI-compatible API endpoint"
        >
          <input
            type="text"
            value={expression_polishing.api_url}
            onChange={(e) =>
              updateExpressionPolishing({ api_url: e.target.value })
            }
            placeholder="https://api.openai.com/v1"
            className="w-64 px-3 py-1.5 text-sm bg-gray-800 border border-gray-600 rounded text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
          />
        </SettingRow>

        {/* API Key */}
        <SettingRow
          label="API Key"
          description="Your OpenAI API key (stored locally)"
        >
          <PasswordInput
            value={expression_polishing.api_key}
            onChange={(value) => updateExpressionPolishing({ api_key: value })}
            placeholder="sk-..."
          />
        </SettingRow>

        {/* Model */}
        <SettingRow label="Model" description="AI model to use for polishing">
          <input
            type="text"
            value={expression_polishing.model}
            onChange={(e) =>
              updateExpressionPolishing({ model: e.target.value })
            }
            placeholder="gpt-4"
            className="w-48 px-3 py-1.5 text-sm bg-gray-800 border border-gray-600 rounded text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
          />
        </SettingRow>
      </div>

      {/* System Prompt */}
      <div className="pt-4 border-t border-white/10">
        <h3 className="text-sm font-medium text-white mb-2">System Prompt</h3>
        <p className="text-xs text-gray-400 mb-3">
          Customize the instructions given to the AI for polishing expressions.
        </p>
        <textarea
          value={expression_polishing.system_prompt}
          onChange={(e) =>
            updateExpressionPolishing({ system_prompt: e.target.value })
          }
          rows={8}
          className="w-full px-3 py-2 text-sm bg-gray-800 border border-gray-600 rounded text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 resize-y"
        />
      </div>

      {/* Status */}
      <div className="pt-4 border-t border-white/10">
        <div className="flex items-center gap-2">
          <div
            className={`w-2 h-2 rounded-full ${
              expression_polishing.api_key ? "bg-green-500" : "bg-yellow-500"
            }`}
          />
          <span className="text-sm text-gray-400">
            {expression_polishing.api_key
              ? "API key configured"
              : "API key not set - polishing will not work"}
          </span>
        </div>
      </div>
    </div>
  );
}

