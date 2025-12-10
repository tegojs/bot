import { useState } from "react";
import { useSettingsStore } from "@/stores/settingsStore";
import { useDialogueStore } from "@/stores/dialogueStore";
import { Eye, EyeOff, Trash2 } from "lucide-react";

export function AIDialogueSettings() {
  const { settings, updateAIDialogue } = useSettingsStore();
  const { ai_dialogue } = settings;
  const { conversations, clearAllConversations } = useDialogueStore();
  const [showApiKey, setShowApiKey] = useState(false);

  const handleClearConversations = () => {
    if (
      window.confirm(
        `Are you sure you want to delete all ${conversations.length} conversations? This cannot be undone.`
      )
    ) {
      clearAllConversations();
    }
  };

  return (
    <div className="space-y-8">
      <div>
        <h2 className="text-lg font-semibold text-white mb-1">AI Dialogue</h2>
        <p className="text-sm text-gray-400">
          Configure the AI assistant for multi-turn conversations.
        </p>
      </div>

      <div className="space-y-6">
        {/* API URL */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            API URL
          </label>
          <input
            type="text"
            value={ai_dialogue.api_url}
            onChange={(e) => updateAIDialogue({ api_url: e.target.value })}
            placeholder="https://api.openai.com/v1"
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
          />
          <p className="text-xs text-gray-500 mt-1">
            OpenAI-compatible API endpoint
          </p>
        </div>

        {/* API Key */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            API Key
          </label>
          <div className="flex gap-2">
            <input
              type={showApiKey ? "text" : "password"}
              value={ai_dialogue.api_key}
              onChange={(e) => updateAIDialogue({ api_key: e.target.value })}
              placeholder="sk-..."
              className="flex-1 bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
            />
            <button
              type="button"
              onClick={() => setShowApiKey(!showApiKey)}
              className="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
            >
              {showApiKey ? (
                <EyeOff className="w-4 h-4" />
              ) : (
                <Eye className="w-4 h-4" />
              )}
            </button>
          </div>
          <p className="text-xs text-gray-500 mt-1">
            Your API key is stored locally and never shared
          </p>
        </div>

        {/* Model */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Model
          </label>
          <input
            type="text"
            value={ai_dialogue.model}
            onChange={(e) => updateAIDialogue({ model: e.target.value })}
            placeholder="gpt-4"
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
          />
          <p className="text-xs text-gray-500 mt-1">
            Model to use for conversations (e.g., gpt-4, gpt-3.5-turbo)
          </p>
        </div>

        {/* Max History Messages */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Max History Messages: {ai_dialogue.max_history_messages}
          </label>
          <input
            type="range"
            min="5"
            max="50"
            step="5"
            value={ai_dialogue.max_history_messages}
            onChange={(e) =>
              updateAIDialogue({ max_history_messages: Number(e.target.value) })
            }
            className="w-full accent-emerald-500"
          />
          <p className="text-xs text-gray-500 mt-1">
            Number of recent messages to include as context (affects token usage)
          </p>
        </div>

        {/* System Prompt */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            System Prompt
          </label>
          <textarea
            value={ai_dialogue.system_prompt}
            onChange={(e) => updateAIDialogue({ system_prompt: e.target.value })}
            rows={4}
            placeholder="You are a helpful assistant."
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-transparent resize-none"
          />
          <p className="text-xs text-gray-500 mt-1">
            Instructions that define the AI's behavior and personality
          </p>
        </div>

        {/* Status Indicator */}
        <div className="pt-4 border-t border-white/10">
          <div className="flex items-center gap-2">
            <div
              className={`w-2 h-2 rounded-full ${
                ai_dialogue.api_key ? "bg-emerald-500" : "bg-yellow-500"
              }`}
            />
            <span className="text-sm text-gray-400">
              {ai_dialogue.api_key
                ? "API key configured"
                : "API key not set - dialogue will not work"}
            </span>
          </div>
        </div>

        {/* Clear Conversations */}
        {conversations.length > 0 && (
          <div className="pt-4 border-t border-white/10">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium text-gray-300">
                  Clear All Conversations
                </div>
                <p className="text-xs text-gray-500 mt-0.5">
                  Delete all {conversations.length} conversation
                  {conversations.length === 1 ? "" : "s"}
                </p>
              </div>
              <button
                type="button"
                onClick={handleClearConversations}
                className="flex items-center gap-2 px-3 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-lg transition-colors"
              >
                <Trash2 className="w-4 h-4" />
                <span className="text-sm">Clear All</span>
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
