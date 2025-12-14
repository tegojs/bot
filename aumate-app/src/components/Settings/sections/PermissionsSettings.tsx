import { invoke } from "@tauri-apps/api/core";
import { Mic, MousePointer, Shield, Video } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { cn } from "@/lib/utils";

interface PermissionStatus {
  screen_recording: boolean;
  accessibility: boolean;
  microphone: boolean;
}

export function PermissionsSettings() {
  const [permissions, setPermissions] = useState<PermissionStatus>({
    screen_recording: false,
    accessibility: false,
    microphone: false,
  });
  const [loading, setLoading] = useState(true);

  const checkPermissions = useCallback(async () => {
    try {
      const status = await invoke<PermissionStatus>("check_permissions");
      setPermissions(status);
    } catch (error) {
      console.error("Failed to check permissions:", error);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    checkPermissions();
    // 定期检查权限状态
    const interval = setInterval(checkPermissions, 3000);
    return () => clearInterval(interval);
  }, [checkPermissions]);

  const handleRequestScreenRecording = async () => {
    try {
      await invoke("request_screen_recording_permission");
      // 等待一下再检查状态
      setTimeout(checkPermissions, 1000);
    } catch (error) {
      console.error("Failed to request screen recording permission:", error);
    }
  };

  const handleRequestAccessibility = async () => {
    try {
      await invoke("request_accessibility_permission");
      // 权限请求会打开系统设置，提示用户手动启用
    } catch (error) {
      console.error("Failed to request accessibility permission:", error);
    }
  };

  const handleRequestMicrophone = async () => {
    try {
      await invoke("request_microphone_permission");
      // 等待一下再检查状态
      setTimeout(checkPermissions, 1000);
    } catch (error) {
      console.error("Failed to request microphone permission:", error);
    }
  };

  if (loading) {
    return (
      <div className="space-y-6">
        <h2 className="text-xl font-semibold text-white">Permissions</h2>
        <div className="text-sm text-gray-400">Checking permissions...</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-white">Permissions</h2>
        <p className="text-sm text-gray-400 mt-2">
          Aumate needs certain permissions to work correctly. Click on each
          permission to enable it.
        </p>
      </div>

      <div className="space-y-3">
        <PermissionCard
          icon={<Video className="w-5 h-5" />}
          title="录屏与系统录音"
          description="截图功能将使用该权限用以获取屏幕画面"
          enabled={permissions.screen_recording}
          onRequest={handleRequestScreenRecording}
          color="blue"
        />

        <PermissionCard
          icon={<MousePointer className="w-5 h-5" />}
          title="辅助功能"
          description="通过辅助功能以实现鼠标移动、录入按键等操作"
          enabled={permissions.accessibility}
          onRequest={handleRequestAccessibility}
          color="purple"
        />

        <PermissionCard
          icon={<Mic className="w-5 h-5" />}
          title="麦克风"
          description="在视频录制时，将使用该权限用以录制麦克风声音"
          enabled={permissions.microphone}
          onRequest={handleRequestMicrophone}
          color="emerald"
        />
      </div>

      <div className="flex items-start gap-2 p-4 bg-blue-500/10 border border-blue-500/30 rounded-lg">
        <Shield className="w-5 h-5 text-blue-400 flex-shrink-0 mt-0.5" />
        <div className="text-xs text-blue-300">
          <p className="font-medium mb-1">Privacy Notice</p>
          <p className="text-blue-200/80">
            These permissions are only used for the features described. Aumate
            does not collect or transmit any data without your explicit consent.
          </p>
        </div>
      </div>
    </div>
  );
}

interface PermissionCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  enabled: boolean;
  onRequest: () => void;
  color: "blue" | "purple" | "emerald";
}

function PermissionCard({
  icon,
  title,
  description,
  enabled,
  onRequest,
  color,
}: PermissionCardProps) {
  const colorClasses = {
    blue: {
      bg: "bg-blue-500/10",
      border: "border-blue-500/30",
      icon: "text-blue-400",
      badge: "bg-blue-500/20 text-blue-300",
      button: "bg-blue-600 hover:bg-blue-700",
    },
    purple: {
      bg: "bg-purple-500/10",
      border: "border-purple-500/30",
      icon: "text-purple-400",
      badge: "bg-purple-500/20 text-purple-300",
      button: "bg-purple-600 hover:bg-purple-700",
    },
    emerald: {
      bg: "bg-emerald-500/10",
      border: "border-emerald-500/30",
      icon: "text-emerald-400",
      badge: "bg-emerald-500/20 text-emerald-300",
      button: "bg-emerald-600 hover:bg-emerald-700",
    },
  };

  const colors = colorClasses[color];

  return (
    <div
      className={cn(
        "flex items-start justify-between p-4 rounded-lg border transition-all",
        enabled
          ? `${colors.bg} ${colors.border}`
          : "bg-gray-800/50 border-gray-700 hover:border-gray-600",
      )}
    >
      <div className="flex items-start gap-3 flex-1">
        <div
          className={cn(
            "p-2.5 rounded-md flex-shrink-0",
            enabled ? colors.bg : "bg-gray-700",
            colors.icon,
          )}
        >
          {icon}
        </div>
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <div className="text-sm font-medium text-white">{title}</div>
            {enabled && (
              <span
                className={cn(
                  "px-2 py-0.5 text-xs font-medium rounded",
                  colors.badge,
                )}
              >
                已启用
              </span>
            )}
          </div>
          <div className="text-xs text-gray-400">{description}</div>
        </div>
      </div>
      {!enabled && (
        <button
          type="button"
          onClick={onRequest}
          className={cn(
            "ml-3 px-4 py-2 text-xs font-medium text-white rounded-lg transition-colors flex-shrink-0",
            colors.button,
          )}
        >
          启用
        </button>
      )}
    </div>
  );
}
