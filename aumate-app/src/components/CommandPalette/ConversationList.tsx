import { MessageSquare, Plus, Trash2 } from "lucide-react";
import { cn } from "@/lib/utils";
import type { Conversation } from "@/types/dialogue";

interface ConversationListProps {
  conversations: Conversation[];
  activeConversationId: string | null;
  onSelectConversation: (id: string) => void;
  onNewConversation: () => void;
  onDeleteConversation: (id: string) => void;
}

// Group conversations by date
function groupByDate(conversations: Conversation[]): Map<string, Conversation[]> {
  const groups = new Map<string, Conversation[]>();
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime();
  const yesterday = today - 86400000;
  const lastWeek = today - 7 * 86400000;

  for (const conv of conversations) {
    const convDate = new Date(conv.updatedAt);
    const convDay = new Date(
      convDate.getFullYear(),
      convDate.getMonth(),
      convDate.getDate()
    ).getTime();

    let label: string;
    if (convDay >= today) {
      label = "Today";
    } else if (convDay >= yesterday) {
      label = "Yesterday";
    } else if (convDay >= lastWeek) {
      label = "Last 7 Days";
    } else {
      label = convDate.toLocaleDateString("en-US", {
        month: "short",
        year: "numeric",
      });
    }

    const existing = groups.get(label) || [];
    existing.push(conv);
    groups.set(label, existing);
  }

  return groups;
}

export function ConversationList({
  conversations,
  activeConversationId,
  onSelectConversation,
  onNewConversation,
  onDeleteConversation,
}: ConversationListProps) {
  const grouped = groupByDate(conversations);

  return (
    <div className="w-52 border-r border-white/10 flex flex-col h-full">
      {/* Header */}
      <div className="px-3 py-2 border-b border-white/10">
        <button
          type="button"
          onClick={onNewConversation}
          className="w-full flex items-center gap-2 px-3 py-2 text-sm text-muted-foreground hover:text-foreground hover:bg-white/5 rounded-lg transition-colors"
        >
          <Plus className="w-4 h-4" />
          <span>New Chat</span>
        </button>
      </div>

      {/* Conversation List */}
      <div className="flex-1 overflow-y-auto py-2">
        {conversations.length === 0 ? (
          <div className="px-3 py-8 text-center text-muted-foreground text-xs">
            <MessageSquare className="w-6 h-6 mx-auto mb-2 opacity-50" />
            <p>No conversations yet</p>
          </div>
        ) : (
          Array.from(grouped.entries()).map(([label, convs]) => (
            <div key={label} className="mb-2">
              <div className="px-3 py-1 text-xs text-muted-foreground/60 font-medium">
                {label}
              </div>
              {convs.map((conv) => (
                <div
                  key={conv.id}
                  className={cn(
                    "group relative mx-2 rounded-lg transition-colors",
                    activeConversationId === conv.id
                      ? "bg-accent"
                      : "hover:bg-white/5"
                  )}
                >
                  <button
                    type="button"
                    onClick={() => onSelectConversation(conv.id)}
                    className="w-full text-left px-3 py-2"
                  >
                    <div
                      className={cn(
                        "text-sm truncate",
                        activeConversationId === conv.id
                          ? "text-accent-foreground"
                          : "text-foreground"
                      )}
                    >
                      {conv.title}
                    </div>
                    <div className="text-xs text-muted-foreground truncate">
                      {conv.messages.length} messages
                    </div>
                  </button>
                  <button
                    type="button"
                    onClick={(e) => {
                      e.stopPropagation();
                      onDeleteConversation(conv.id);
                    }}
                    className="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-muted-foreground hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity"
                    title="Delete conversation"
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                  </button>
                </div>
              ))}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
