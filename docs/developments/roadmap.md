# Tego Bot Development Roadmap

## Technology Stack Planning

### Phase 1: egui + winit (Basic UI Framework)
- **Goal**: Establish basic window system and UI component library
- **Technology Selection**:
  - `egui` - Immediate mode GUI framework, suitable for procedural development
  - `winit` - Cross-platform window management
  - `softbuffer` - CPU rendering backend (lightweight)
- **Use Cases**: Basic UI components, simple animations, floating window basics

### Phase 2: egui + wgpu (Advanced Rendering and Effects)
- **Goal**: Support complex graphics effects and GPU-accelerated rendering
- **Technology Selection**:
  - `egui` - Maintain UI framework consistency
  - `wgpu` - GPU-accelerated rendering
  - `egui-wgpu` - egui and wgpu integration
- **Use Cases**: Complex animations, advanced graphics rendering, high-performance scenarios

---

## Detailed Feature Module Design

### 1. Screen Capture & Annotation

#### Feature Description
Provides a complete screenshot toolchain, including:
- Full screen/region capture
- Interactive region selection (mouse drag selection)
- **Window snapping**: Selection can auto-snap to window edges
- **Selection adjustment**: Support dragging edges and corners to resize
- **Selection change callback**: Real-time coordinates and snapped window info
- Post-capture annotation (brush, arrow, text, rectangle, circle, etc.)
- Pixel color picker
- Save screenshot (supports PNG, JPG, WebP formats, specified at save time)
- Clipboard operations
- Post-capture processing callbacks (OCR, recognition, etc.)

#### TypeScript API Design

```typescript
// ============================================================================
// Screenshot Tool Core API
// ============================================================================

/**
 * Screenshot Tool Class - Provides complete capture and annotation functionality
 */
export class ScreenshotTool {
  /**
   * Create screenshot tool instance
   * @param options - Configuration options
   */
  constructor(options?: ScreenshotToolOptions);

  /**
   * Start interactive capture (similar to Snipaste)
   * Shows fullscreen overlay, user can drag to select region
   * Supports window snapping, selection adjustment, real-time callbacks
   * @param options - Capture options
   * @returns Promise<ScreenshotResult> - Capture result
   */
  captureInteractive(options?: InteractiveCaptureOptions): Promise<ScreenshotResult>;

  /**
   * Get current selection info (during interactive capture)
   * @returns SelectionInfo | null - Current selection info, null if not in capture process
   */
  getCurrentSelection(): SelectionInfo | null;

  /**
   * Quick capture (directly capture fullscreen or specified region, no interaction)
   * @param region - Optional, specify capture region
   * @returns Promise<ScreenshotResult>
   */
  captureQuick(region?: ScreenRegion): Promise<ScreenshotResult>;

  /**
   * Open annotation editor
   * Add annotations to screenshot (brush, arrow, text, etc.)
   * @param screenshot - Screenshot result
   * @param options - Editor options
   * @returns Promise<AnnotatedScreenshot> - Annotated screenshot
   */
  annotate(
    screenshot: ScreenshotResult,
    options?: AnnotationEditorOptions
  ): Promise<AnnotatedScreenshot>;

  /**
   * Get pixel color at specified screen position
   * @param x - X coordinate
   * @param y - Y coordinate
   * @returns Promise<ColorInfo> - Color information
   */
  getPixelColor(x: number, y: number): Promise<ColorInfo>;

  /**
   * Start color picker (real-time display of color at mouse position)
   * @param options - Picker options
   * @returns Promise<ColorInfo> - Selected color
   */
  pickColor(options?: ColorPickerOptions): Promise<ColorInfo>;

  /**
   * Close screenshot tool, cleanup resources
   */
  close(): Promise<void>;
}

/**
 * Screenshot tool configuration options
 */
export interface ScreenshotToolOptions {
  /** Default save path (optional) */
  defaultSavePath?: string;
  /** Auto copy to clipboard */
  autoCopyToClipboard?: boolean;
  /** Post-capture callback */
  onCaptureComplete?: (result: ScreenshotResult) => void | Promise<void>;
}

/**
 * Interactive capture options
 */
export interface InteractiveCaptureOptions {
  /** Show grid guide lines */
  showGrid?: boolean;
  /** Show coordinate info */
  showCoordinates?: boolean;
  /** Show size info */
  showSize?: boolean;
  /** Hint text during selection */
  hintText?: string;
  /** Enable window snapping */
  enableWindowSnap?: boolean;
  /** Window snap threshold (pixels, default 10) */
  snapThreshold?: number;
  /** Selection change callback */
  onSelectionChange?: (info: SelectionInfo) => void;
  /** Hotkey configuration */
  hotkeys?: {
    /** Confirm capture (default Enter) */
    confirm?: string;
    /** Cancel capture (default Escape) */
    cancel?: string;
    /** Toggle fullscreen mode (default F) */
    toggleFullscreen?: string;
  };
}

/**
 * Screenshot result
 */
export interface ScreenshotResult {
  /** Screenshot image data (PNG Buffer, original format) */
  image: Buffer;
  /** Screenshot region info */
  region: ScreenRegion;
  /** Screenshot timestamp */
  timestamp: number;
  /** Save to file (can specify format and quality) */
  saveToFile(
    path: string,
    options?: SaveImageOptions
  ): Promise<void>;
  /** Copy to clipboard */
  copyToClipboard?(): Promise<void>;
}

/**
 * Save image options
 */
export interface SaveImageOptions {
  /** Image format */
  format?: 'png' | 'jpg' | 'webp';
  /** Image quality (1-100, only for jpg/webp, default 90) */
  quality?: number;
}

/**
 * Selection info (for callbacks)
 */
export interface SelectionInfo {
  /** Current selection region */
  region: ScreenRegion;
  /** Whether snapped to window */
  isSnapped: boolean;
  /** Snapped window info (if snapped) */
  snappedWindow?: WindowSnapInfo;
  /** Whether can resize */
  canResize: boolean;
}

/**
 * Window snap info
 */
export interface WindowSnapInfo {
  /** Window ID */
  windowId: string;
  /** Window title */
  title: string;
  /** Window region */
  region: ScreenRegion;
  /** Snap edge ('left' | 'right' | 'top' | 'bottom' | 'all') */
  snapEdge: string;
}

/**
 * Screen region
 */
export interface ScreenRegion {
  x: number;
  y: number;
  width: number;
  height: number;
}

/**
 * Annotation editor options
 */
export interface AnnotationEditorOptions {
  /** Initial tool (default 'arrow') */
  defaultTool?: AnnotationTool;
  /** Brush color */
  brushColor?: string;
  /** Brush size */
  brushSize?: number;
  /** Show toolbar */
  showToolbar?: boolean;
  /** Available annotation tools */
  availableTools?: AnnotationTool[];
}

/**
 * Annotation tool types
 */
export type AnnotationTool =
  | 'arrow'      // Arrow
  | 'brush'      // Brush
  | 'rectangle'  // Rectangle
  | 'circle'     // Circle
  | 'text'       // Text
  | 'blur'       // Blur/Mosaic
  | 'highlight'  // Highlight
  | 'eraser';    // Eraser

/**
 * Annotated screenshot
 */
export interface AnnotatedScreenshot extends ScreenshotResult {
  /** Annotation layer data (for undo/redo) */
  annotations: AnnotationLayer[];
}

/**
 * Annotation layer
 */
export interface AnnotationLayer {
  id: string;
  type: AnnotationTool;
  data: unknown; // Varies by tool type
  timestamp: number;
}

/**
 * Color info
 */
export interface ColorInfo {
  /** RGB values */
  rgb: { r: number; g: number; b: number };
  /** RGBA values */
  rgba: { r: number; g: number; b: number; a: number };
  /** HEX format */
  hex: string;
  /** HSL values */
  hsl: { h: number; s: number; l: number };
  /** Coordinate position */
  position: { x: number; y: number };
}

/**
 * Color picker options
 */
export interface ColorPickerOptions {
  /** Magnifier size */
  magnifierSize?: number;
  /** Zoom level */
  zoom?: number;
  /** Show color history */
  showHistory?: boolean;
}

// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Quick screenshot (fullscreen)
 */
export function quickScreenshot(): Promise<ScreenshotResult>;

/**
 * Quick screenshot (specified region)
 */
export function quickScreenshotRegion(region: ScreenRegion): Promise<ScreenshotResult>;

/**
 * Start interactive capture
 */
export function startInteractiveCapture(
  options?: InteractiveCaptureOptions
): Promise<ScreenshotResult>;
```

---

### 2. Long Screenshot / Scrolling Capture

#### Feature Description
Provides automatic scrolling screenshot functionality, supporting:
- Auto-detect scrollable areas
- Vertical/horizontal scrolling capture
- Multi-display stitching
- Scroll speed control
- Smart deduplication (remove overlapping parts)

#### TypeScript API Design

```typescript
// ============================================================================
// Long Screenshot API
// ============================================================================

/**
 * Long Screenshot Tool Class
 */
export class LongScreenshotTool {
  /**
   * Create long screenshot tool instance
   */
  constructor(options?: LongScreenshotOptions);

  /**
   * Vertical long screenshot (scroll down)
   * @param options - Long screenshot options
   * @returns Promise<LongScreenshotResult>
   */
  captureVertical(options?: VerticalCaptureOptions): Promise<LongScreenshotResult>;

  /**
   * Horizontal long screenshot (scroll right)
   * @param options - Long screenshot options
   * @returns Promise<LongScreenshotResult>
   */
  captureHorizontal(options?: HorizontalCaptureOptions): Promise<LongScreenshotResult>;

  /**
   * Custom direction long screenshot
   * @param direction - Scroll direction
   * @param options - Long screenshot options
   * @returns Promise<LongScreenshotResult>
   */
  capture(
    direction: ScrollDirection,
    options?: LongScreenshotOptions
  ): Promise<LongScreenshotResult>;

  /**
   * Cancel ongoing capture
   */
  cancel(): Promise<void>;
}

/**
 * Long screenshot configuration options
 */
export interface LongScreenshotOptions {
  /** Target region (auto-detect if not specified) */
  region?: ScreenRegion;
  /** Scroll speed (pixels/second) */
  scrollSpeed?: number;
  /** Wait time after each scroll (ms) */
  scrollDelay?: number;
  /** Max screenshot length (pixels, prevent infinite scroll) */
  maxLength?: number;
  /** Enable smart deduplication (detect and remove overlapping parts) */
  enableDeduplication?: boolean;
  /** Progress callback */
  onProgress?: (progress: LongScreenshotProgress) => void;
}

/**
 * Vertical capture options
 */
export interface VerticalCaptureOptions extends LongScreenshotOptions {
  /** Scroll step (pixels) */
  scrollStep?: number;
  /** Auto-detect scroll area */
  autoDetectScrollArea?: boolean;
}

/**
 * Horizontal capture options
 */
export interface HorizontalCaptureOptions extends LongScreenshotOptions {
  /** Scroll step (pixels) */
  scrollStep?: number;
  /** Auto-detect scroll area */
  autoDetectScrollArea?: boolean;
}

/**
 * Scroll direction
 */
export type ScrollDirection = 'up' | 'down' | 'left' | 'right';

/**
 * Long screenshot result
 */
export interface LongScreenshotResult {
  /** Stitched complete image */
  image: Buffer;
  /** Final dimensions */
  dimensions: { width: number; height: number };
  /** Number of original segments */
  segmentCount: number;
  /** Capture duration (ms) */
  duration: number;
  /** Save to file */
  saveToFile(path: string): Promise<void>;
}

/**
 * Long screenshot progress
 */
export interface LongScreenshotProgress {
  /** Current progress (0-1) */
  progress: number;
  /** Captured segments */
  segments: number;
  /** Current total length (pixels) */
  currentLength: number;
  /** Estimated total length (pixels, may be undefined) */
  estimatedLength?: number;
}

// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Quick vertical long screenshot
 */
export function captureLongScreenshotVertical(
  options?: VerticalCaptureOptions
): Promise<LongScreenshotResult>;

/**
 * Quick horizontal long screenshot
 */
export function captureLongScreenshotHorizontal(
  options?: HorizontalCaptureOptions
): Promise<LongScreenshotResult>;
```

---

### 3. Floating Window System

#### Feature Description
Provides ability to create and manage floating windows, supporting:
- Create borderless, draggable floating windows
- **Window shapes**: Support rectangle, circle, custom shape (transparent image mask)
- **Custom shape windows**: Use transparent images to define window shape, mouse only responds in non-transparent areas
- **Window icons**: Support Emoji, preset icons (enum), custom images
- **Edge particle effects**: Built-in 6 edge particle effects (Rotating Halo, Pulse Ripple, Flowing Light, Stardust Scatter, Electric Spark, Smoke Wisp)
- Dynamically replace window content (image, text, custom UI)
- Window animations (fade, scale, slide animation)
- Window level management
- Multi-window management
- Window event handling

#### Edge Particle Effects Detailed Description

All particle effects generate and move along window edges, forming visual effects surrounding the window:

**1. Rotating Halo**
- **Visual Effect**: Particles rotate clockwise or counterclockwise along window edge, forming continuous halo effect
- **Algorithm Implementation**:
  - Particles distributed evenly along window edge using polar coordinate system
  - Position calculation: `θ(t) = θ₀ + ω·t`, where `ω` is angular velocity (radians/second)
  - Coordinate conversion: `x = center_x + r·cos(θ)`, `y = center_y + r·sin(θ)`
  - Use `sin` and `cos` functions for smooth circular motion
  - Particle color can vary with angle, use `sin(θ + phase)` to control color intensity
- **Parameters**: Rotation speed (ω), particle density, color gradient

**2. Pulse Ripple**
- **Visual Effect**: Particles expand outward from window edge, forming concentric circle ripples, like water waves
- **Algorithm Implementation**:
  - Particles start from edge position, move radially outward
  - Radial distance: `r(t) = r₀ + v·t + A·sin(2πft)`, where `A` is amplitude, `f` is frequency
  - Opacity decay: `α(t) = α₀·(1 - t/T)`, `T` is lifetime
  - Use `sin` function for periodic ripple variation
  - Multiple ripple layers overlap, use phase difference `φ = 2π·n/N` for continuous ripples
- **Parameters**: Ripple speed, ripple frequency, decay speed

**3. Flowing Light**
- **Visual Effect**: Particles flow along window edge, forming continuous light flow effect, like neon lights
- **Algorithm Implementation**:
  - Particles move along edge path using parameterized curves
  - Path parameter: `s(t) = (s₀ + v·t) mod L`, `L` is total edge length
  - Brightness variation: `brightness = (sin(2π·s/λ + phase) + 1) / 2`, `λ` is wavelength
  - Use `sin` function for flowing light brightness gradient
  - Multiple particles form continuous light band, phase difference creates flow feeling
- **Parameters**: Flow speed, light length, color gradient

**4. Stardust Scatter**
- **Visual Effect**: Particles generate at random positions along window edge, scatter in random directions, forming stardust effect
- **Algorithm Implementation**:
  - Particles generate at random edge positions: `θ = random(0, 2π)`
  - Movement direction: `φ = random(0, 2π)`, speed: `v = v₀ + random(-Δv, Δv)`
  - Position update: `x(t) = x₀ + v·cos(φ)·t`, `y(t) = y₀ + v·sin(φ)·t`
  - Opacity decay: `α(t) = α₀·exp(-t/τ)`, `τ` is decay time constant
  - Use random functions and trigonometric functions for natural scattering
- **Parameters**: Generation frequency, scatter speed range, lifetime

**5. Electric Spark**
- **Visual Effect**: Particles flash and jump rapidly along window edge, forming electric current effect
- **Algorithm Implementation**:
  - Particles rapidly generate and disappear at random edge positions
  - Position jumping: Use Perlin noise or random functions to generate paths
  - Brightness flashing: `brightness = random() > threshold ? 1.0 : 0.0`
  - Fast decay: `α(t) = α₀·(1 - t/T_fast)`, `T_fast` is fast decay time
  - Use `sin` function overlay for flash frequency control
  - Branch effect: Particles can split into multiple sub-particles
- **Parameters**: Flash frequency, jump distance, branch probability

**6. Smoke Wisp**
- **Visual Effect**: Particles slowly rise and scatter from window edge, forming smoke effect
- **Algorithm Implementation**:
  - Particles generate from bottom edge, move upward
  - Vertical velocity: `v_y = v₀·(1 - t/T)`, gradually slowing
  - Horizontal drift: `x(t) = x₀ + A·sin(ω·t + φ)`, use `sin` function for left-right swaying
  - Opacity gradient: `α(t) = α₀·(1 - (t/T)^2)`, non-linear decay
  - Size change: `size(t) = size₀·(1 + t/T)`, gradually growing
  - Use `cos` function to control smoke distortion effect
- **Parameters**: Rise speed, drift amplitude, diffusion speed

#### TypeScript API Design

```typescript
// ============================================================================
// Floating Window API
// ============================================================================

/**
 * Floating Window Class
 */
export class FloatingWindow {
  /**
   * Create floating window
   * @param options - Window configuration
   */
  constructor(options: FloatingWindowOptions);

  /**
   * Show window
   * @param animation - Optional, show animation
   */
  show(animation?: WindowAnimation): Promise<void>;

  /**
   * Hide window
   * @param animation - Optional, hide animation
   */
  hide(animation?: WindowAnimation): Promise<void>;

  /**
   * Close window (release resources)
   */
  close(): Promise<void>;

  /**
   * Set window position
   * @param x - X coordinate
   * @param y - Y coordinate
   * @param animated - Use animation
   */
  setPosition(x: number, y: number, animated?: boolean): Promise<void>;

  /**
   * Set window size
   * @param width - Width
   * @param height - Height
   * @param animated - Use animation
   */
  setSize(width: number, height: number, animated?: boolean): Promise<void>;

  /**
   * Set window shape
   * @param shape - Window shape configuration
   */
  setShape(shape: WindowShape): Promise<void>;

  /**
   * Set window content (image)
   * @param image - Image Buffer or path
   * @param options - Display options
   */
  setImage(
    image: Buffer | string,
    options?: ImageDisplayOptions
  ): Promise<void>;

  /**
   * Set window content (custom UI)
   * @param renderer - Render function
   */
  setContent(renderer: WindowContentRenderer): void;

  /**
   * Update window content
   */
  update(): Promise<void>;

  /**
   * Set window level
   * @param level - Level ('normal' | 'top' | 'always-top')
   */
  setLevel(level: WindowLevel): Promise<void>;

  /**
   * Set window opacity
   * @param opacity - Opacity (0-1)
   * @param animated - Use animation
   */
  setOpacity(opacity: number, animated?: boolean): Promise<void>;

  /**
   * Set window icon
   * @param icon - Icon configuration
   */
  setIcon(icon: WindowIcon): Promise<void>;

  /**
   * Apply preset effect
   * @param effect - Effect type
   * @param options - Effect options
   */
  applyPresetEffect(
    effect: PresetWindowEffect,
    options?: PresetEffectOptions
  ): Promise<void>;

  /**
   * Get window info
   */
  getInfo(): FloatingWindowInfo;

  /**
   * Event listeners
   */
  on(event: FloatingWindowEvent, handler: EventHandler): void;
  off(event: FloatingWindowEvent, handler: EventHandler): void;
}

/**
 * Floating window configuration
 */
export interface FloatingWindowOptions {
  /** Window ID (unique identifier) */
  id?: string;
  /** Initial position */
  position?: { x: number; y: number };
  /** Initial size */
  size?: { width: number; height: number };
  /** Window shape */
  shape?: WindowShape;
  /** Draggable */
  draggable?: boolean;
  /** Resizable */
  resizable?: boolean;
  /** Click through (mouse events pass through to layer below) */
  clickThrough?: boolean;
  /** Window level */
  level?: WindowLevel;
  /** Initial opacity */
  opacity?: number;
  /** Always on top */
  alwaysOnTop?: boolean;
  /** Window title (for debugging) */
  title?: string;
  /** Window icon */
  icon?: WindowIcon;
  /** Initial content */
  content?: WindowContent;
  /** Preset effect (optional, use built-in effects) */
  presetEffect?: PresetWindowEffect;
}

/**
 * Window level
 */
export type WindowLevel = 'normal' | 'top' | 'always-top';

/**
 * Window content type
 */
export type WindowContent =
  | { type: 'image'; data: Buffer | string; options?: ImageDisplayOptions }
  | { type: 'text'; data: string; options?: TextDisplayOptions }
  | { type: 'custom'; renderer: WindowContentRenderer };

/**
 * Image display options
 */
export interface ImageDisplayOptions {
  /** Scale mode */
  scaleMode?: 'fit' | 'fill' | 'stretch' | 'center';
  /** Background color */
  backgroundColor?: string;
  /** Maintain aspect ratio */
  maintainAspectRatio?: boolean;
}

/**
 * Text display options
 */
export interface TextDisplayOptions {
  /** Font size */
  fontSize?: number;
  /** Font color */
  color?: string;
  /** Background color */
  backgroundColor?: string;
  /** Text alignment */
  align?: 'left' | 'center' | 'right';
  /** Auto wrap */
  wrap?: boolean;
}

/**
 * Window content renderer
 * Define UI using procedural approach
 */
export type WindowContentRenderer = (ui: WindowUI) => void;

/**
 * Window UI API (procedural development interface)
 */
export interface WindowUI {
  /** Draw image */
  image(
    image: Buffer | string,
    x: number,
    y: number,
    width?: number,
    height?: number,
    options?: ImageDisplayOptions
  ): void;

  /** Draw text */
  text(
    text: string,
    x: number,
    y: number,
    options?: TextDisplayOptions
  ): void;

  /** Draw rectangle */
  rect(
    x: number,
    y: number,
    width: number,
    height: number,
    color: string,
    filled?: boolean
  ): void;

  /** Draw circle */
  circle(
    x: number,
    y: number,
    radius: number,
    color: string,
    filled?: boolean
  ): void;

  /** Draw line */
  line(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    color: string,
    width?: number
  ): void;

  /** Clear canvas */
  clear(color?: string): void;

  /** Get canvas size */
  getSize(): { width: number; height: number };
}

/**
 * Window shape configuration
 */
export type WindowShape =
  | { type: 'rectangle' }  // Rectangle (default)
  | { type: 'circle' }      // Circle
  | { type: 'custom'; mask: Buffer | string };  // Custom shape (use transparent image as mask)

/**
 * Window icon configuration
 */
export type WindowIcon =
  | { type: 'emoji'; emoji: string }  // Emoji icon
  | { type: 'icon'; icon: IconName }  // Preset icon (using enum)
  | { type: 'image'; image: Buffer | string };  // Custom image icon

/**
 * Preset icon names (enum)
 * Using third-party icon library (e.g., Material Icons, Font Awesome, etc.)
 */
export enum IconName {
  // Common icons
  CLOSE = 'close',
  MINIMIZE = 'minimize',
  MAXIMIZE = 'maximize',
  SETTINGS = 'settings',
  SEARCH = 'search',
  HOME = 'home',
  BACK = 'back',
  FORWARD = 'forward',
  REFRESH = 'refresh',
  DOWNLOAD = 'download',
  UPLOAD = 'upload',
  SAVE = 'save',
  EDIT = 'edit',
  DELETE = 'delete',
  ADD = 'add',
  REMOVE = 'remove',
  CHECK = 'check',
  CANCEL = 'cancel',
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
  SUCCESS = 'success',
  // More icons...
}

/**
 * Preset window edge particle effects
 * All effects generate and move along window edges
 */
export enum PresetWindowEffect {
  /** Rotating Halo - Particles rotate along window edge forming halo */
  ROTATING_HALO = 'rotating-halo',
  /** Pulse Ripple - Particles expand outward from edge forming ripples */
  PULSE_RIPPLE = 'pulse-ripple',
  /** Flowing Light - Particles flow along edge forming light streams */
  FLOWING_LIGHT = 'flowing-light',
  /** Stardust Scatter - Particles scatter randomly from edge */
  STARDUST_SCATTER = 'stardust-scatter',
  /** Electric Spark - Particles flash and jump rapidly along edge */
  ELECTRIC_SPARK = 'electric-spark',
  /** Smoke Wisp - Particles slowly rise and scatter from edge */
  SMOKE_WISP = 'smoke-wisp',
}

/**
 * Preset effect options
 */
export interface PresetEffectOptions {
  /** Particle count (default auto-calculated based on window size) */
  particleCount?: number;
  /** Particle size (pixels, default 2-4) */
  particleSize?: number | { min: number; max: number };
  /** Particle color (default auto-selected based on effect type) */
  particleColor?: string | string[];
  /** Particle speed (default auto-set based on effect type) */
  particleSpeed?: number;
  /** Effect intensity (0-1, affects particle density and speed, default 0.5) */
  intensity?: number;
  /** Loop playback (default true) */
  loop?: boolean;
  /** Loop count (-1 for infinite, default -1) */
  loopCount?: number;
  /** Edge width (particle generation area, pixels, default 10) */
  edgeWidth?: number;
}

/**
 * Window animation
 */
export interface WindowAnimation {
  /** Animation type */
  type: 'fade' | 'scale' | 'slide' | 'none' | 'bounce' | 'rotate' | 'blink';
  /** Animation duration (ms) */
  duration?: number;
  /** Easing function */
  easing?: EasingFunction;
  /** Animation direction (only for slide) */
  direction?: 'up' | 'down' | 'left' | 'right';
}

/**
 * Easing function types
 */
export type EasingFunction =
  | 'linear'
  | 'ease-in'
  | 'ease-out'
  | 'ease-in-out'
  | 'bounce'
  | 'elastic';

/**
 * Window info
 */
export interface FloatingWindowInfo {
  id: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  shape: WindowShape;
  visible: boolean;
  level: WindowLevel;
  opacity: number;
  icon?: WindowIcon;
}

/**
 * Window event types
 */
export type FloatingWindowEvent =
  | 'show'
  | 'hide'
  | 'close'
  | 'move'
  | 'resize'
  | 'click'
  | 'drag-start'
  | 'drag'
  | 'drag-end';

/**
 * Event handler
 */
export type EventHandler = (event: WindowEventData) => void;

/**
 * Window event data
 */
export interface WindowEventData {
  type: FloatingWindowEvent;
  window: FloatingWindowInfo;
  data?: unknown; // Varies by event type
}

// ============================================================================
// Window Manager
// ============================================================================

/**
 * Floating Window Manager
 */
export class FloatingWindowManager {
  /**
   * Create window
   */
  createWindow(options: FloatingWindowOptions): FloatingWindow;

  /**
   * Get window
   */
  getWindow(id: string): FloatingWindow | undefined;

  /**
   * Close window
   */
  closeWindow(id: string): Promise<void>;

  /**
   * Close all windows
   */
  closeAll(): Promise<void>;

  /**
   * Get all windows
   */
  getAllWindows(): FloatingWindow[];

  /**
   * Set global window configuration
   */
  setGlobalOptions(options: Partial<FloatingWindowOptions>): void;
}

// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Create floating window
 */
export function createFloatingWindow(
  options: FloatingWindowOptions
): FloatingWindow;

/**
 * Get window manager instance
 */
export function getWindowManager(): FloatingWindowManager;
```

---

### 4. Clipboard Manager

#### Feature Description
Provides complete clipboard history management functionality, including:
- Clipboard history (text, images)
- History search and filtering
- Clipboard content categorization and tagging
- Quick access to history
- Clipboard content sync (optional)
- Sensitive information detection and filtering

#### TypeScript API Design

```typescript
// ============================================================================
// Clipboard Management API
// ============================================================================

/**
 * Clipboard Manager
 */
export class ClipboardManager {
  /**
   * Create clipboard manager
   * @param options - Configuration options
   */
  constructor(options?: ClipboardManagerOptions);

  /**
   * Start listening to clipboard changes
   */
  start(): Promise<void>;

  /**
   * Stop listening
   */
  stop(): Promise<void>;

  /**
   * Get clipboard history
   * @param options - Query options
   * @returns Promise<ClipboardHistoryItem[]>
   */
  getHistory(options?: HistoryQueryOptions): Promise<ClipboardHistoryItem[]>;

  /**
   * Get specific history item
   * @param id - Item ID
   * @returns Promise<ClipboardHistoryItem | null>
   */
  getHistoryItem(id: string): Promise<ClipboardHistoryItem | null>;

  /**
   * Search history
   * @param query - Search keyword
   * @param options - Search options
   * @returns Promise<ClipboardHistoryItem[]>
   */
  searchHistory(
    query: string,
    options?: SearchOptions
  ): Promise<ClipboardHistoryItem[]>;

  /**
   * Delete history item
   * @param id - Item ID
   */
  deleteHistoryItem(id: string): Promise<void>;

  /**
   * Clear history
   * @param options - Clear options
   */
  clearHistory(options?: ClearHistoryOptions): Promise<void>;

  /**
   * Paste from history item to clipboard
   * @param id - Item ID
   */
  pasteFromHistory(id: string): Promise<void>;

  /**
   * Add tags to history item
   * @param id - Item ID
   * @param tags - Tag list
   */
  addTags(id: string, tags: string[]): Promise<void>;

  /**
   * Remove tags from history item
   * @param id - Item ID
   * @param tags - Tag list
   */
  removeTags(id: string, tags: string[]): Promise<void>;

  /**
   * Get statistics
   */
  getStatistics(): Promise<ClipboardStatistics>;

  /**
   * Export history
   * @param format - Export format
   * @param path - Export path
   */
  exportHistory(format: 'json' | 'csv', path: string): Promise<void>;

  /**
   * Import history
   * @param path - Import file path
   */
  importHistory(path: string): Promise<void>;

  /**
   * Event listeners
   */
  on(event: ClipboardEvent, handler: ClipboardEventHandler): void;
  off(event: ClipboardEvent, handler: ClipboardEventHandler): void;
}

/**
 * Clipboard manager configuration
 */
export interface ClipboardManagerOptions {
  /** Max history size */
  maxHistorySize?: number;
  /** History storage path */
  storagePath?: string;
  /** Auto save images */
  saveImages?: boolean;
  /** Image storage path */
  imageStoragePath?: string;
  /** Enable sensitive info filtering */
  enableSensitiveFilter?: boolean;
  /** Sensitive info detection patterns */
  sensitivePatterns?: RegExp[];
  /** Enable sync (future feature) */
  enableSync?: boolean;
}

/**
 * History query options
 */
export interface HistoryQueryOptions {
  /** Limit return count */
  limit?: number;
  /** Offset */
  offset?: number;
  /** Filter by type */
  type?: ClipboardItemType;
  /** Filter by tags */
  tags?: string[];
  /** Sort by */
  sortBy?: 'time' | 'size' | 'frequency';
  /** Sort order */
  sortOrder?: 'asc' | 'desc';
}

/**
 * Search options
 */
export interface SearchOptions extends HistoryQueryOptions {
  /** Case sensitive */
  caseSensitive?: boolean;
  /** Use regex */
  useRegex?: boolean;
}

/**
 * Clear history options
 */
export interface ClearHistoryOptions {
  /** Clear image files */
  clearImages?: boolean;
  /** Only clear specified type */
  type?: ClipboardItemType;
}

/**
 * Clipboard history item
 */
export interface ClipboardHistoryItem {
  /** Unique ID */
  id: string;
  /** Content type */
  type: ClipboardItemType;
  /** Content data */
  content: ClipboardContent;
  /** Creation time */
  timestamp: number;
  /** Usage count */
  usageCount: number;
  /** Last used time */
  lastUsed: number;
  /** Tags */
  tags: string[];
  /** Content size (bytes) */
  size: number;
  /** Preview text (for display) */
  preview: string;
}

/**
 * Clipboard content type
 */
export type ClipboardItemType = 'text' | 'image' | 'file' | 'html' | 'rtf';

/**
 * Clipboard content
 */
export type ClipboardContent =
  | { type: 'text'; data: string }
  | { type: 'image'; data: Buffer; format: 'png' | 'jpg' }
  | { type: 'file'; data: string[] } // File path list
  | { type: 'html'; data: string }
  | { type: 'rtf'; data: string };

/**
 * Clipboard statistics
 */
export interface ClipboardStatistics {
  /** Total items */
  totalItems: number;
  /** Text items */
  textItems: number;
  /** Image items */
  imageItems: number;
  /** Total size (bytes) */
  totalSize: number;
  /** Most used items */
  mostUsed: ClipboardHistoryItem[];
  /** Recently used items */
  recentlyUsed: ClipboardHistoryItem[];
}

/**
 * Clipboard event types
 */
export type ClipboardEvent =
  | 'change'        // Clipboard content changed
  | 'item-added'    // New item added to history
  | 'item-deleted'  // Item deleted from history
  | 'item-updated'  // Item updated
  | 'history-cleared'; // History cleared

/**
 * Clipboard event handler
 */
export type ClipboardEventHandler = (event: ClipboardEventData) => void;

/**
 * Clipboard event data
 */
export interface ClipboardEventData {
  type: ClipboardEvent;
  item?: ClipboardHistoryItem;
  data?: unknown;
}

// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Create clipboard manager instance
 */
export function createClipboardManager(
  options?: ClipboardManagerOptions
): ClipboardManager;

/**
 * Get global clipboard manager instance
 */
export function getClipboardManager(): ClipboardManager;
```

---

### 5. Procedural UI Components

#### Feature Description
Provides a procedural development UI component library, supporting:
- List component (scrollable, virtual scrolling)
- Button component
- Input component
- Select component
- Dialog component
- Menu component
- Layout component

#### TypeScript API Design

```typescript
// ============================================================================
// Procedural UI Components API
// ============================================================================

/**
 * UI Context - Core interface for procedural development
 */
export interface UIContext {
  /**
   * Create list component
   */
  list<T>(id: string, items: T[], renderer: ListItemRenderer<T>): ListComponent<T>;

  /**
   * Create button
   */
  button(label: string, onClick: () => void, options?: ButtonOptions): ButtonComponent;

  /**
   * Create input
   */
  input(value: string, onChange: (value: string) => void, options?: InputOptions): InputComponent;

  /**
   * Create select
   */
  select<T>(
    value: T,
    options: T[],
    onChange: (value: T) => void,
    options?: SelectOptions<T>
  ): SelectComponent<T>;

  /**
   * Create dialog
   */
  dialog(options: DialogOptions): DialogComponent;

  /**
   * Create menu
   */
  menu(items: MenuItem[], options?: MenuOptions): MenuComponent;

  /**
   * Layout component
   */
  layout(direction: 'horizontal' | 'vertical', children: Component[]): LayoutComponent;
}

/**
 * List component
 */
export interface ListComponent<T> {
  /** Update list data */
  updateItems(items: T[]): void;
  /** Scroll to item */
  scrollTo(index: number): void;
  /** Get selected item */
  getSelected(): T | null;
  /** Set selected item */
  setSelected(index: number): void;
  /** Event listener */
  on(event: 'select' | 'scroll', handler: (data: unknown) => void): void;
}

/**
 * List item renderer
 */
export type ListItemRenderer<T> = (item: T, index: number) => ListItemContent;

/**
 * List item content
 */
export interface ListItemContent {
  /** Main text */
  text: string;
  /** Subtitle */
  subtitle?: string;
  /** Icon */
  icon?: Buffer | string;
  /** Selectable */
  selectable?: boolean;
}

/**
 * Button component
 */
export interface ButtonComponent {
  /** Set text */
  setText(text: string): void;
  /** Set enabled/disabled */
  setEnabled(enabled: boolean): void;
  /** Click event */
  onClick(handler: () => void): void;
}

/**
 * Button options
 */
export interface ButtonOptions {
  /** Button style */
  style?: 'primary' | 'secondary' | 'danger';
  /** Button size */
  size?: 'small' | 'medium' | 'large';
  /** Disabled */
  disabled?: boolean;
  /** Icon */
  icon?: Buffer | string;
}

/**
 * Input component
 */
export interface InputComponent {
  /** Get value */
  getValue(): string;
  /** Set value */
  setValue(value: string): void;
  /** Set placeholder */
  setPlaceholder(placeholder: string): void;
  /** Set enabled/disabled */
  setEnabled(enabled: boolean): void;
  /** Focus */
  focus(): void;
  /** Blur */
  blur(): void;
}

/**
 * Input options
 */
export interface InputOptions {
  /** Placeholder text */
  placeholder?: string;
  /** Input type */
  type?: 'text' | 'password' | 'number' | 'email';
  /** Multiline */
  multiline?: boolean;
  /** Max length */
  maxLength?: number;
  /** Read only */
  readOnly?: boolean;
}

/**
 * Select component
 */
export interface SelectComponent<T> {
  /** Get selected value */
  getValue(): T | null;
  /** Set selected value */
  setValue(value: T): void;
  /** Update options list */
  updateOptions(options: T[]): void;
}

/**
 * Select options
 */
export interface SelectOptions<T> {
  /** Label extractor function */
  getLabel?: (item: T) => string;
  /** Searchable */
  searchable?: boolean;
  /** Placeholder */
  placeholder?: string;
}

/**
 * Dialog component
 */
export interface DialogComponent {
  /** Show dialog */
  show(): Promise<void>;
  /** Hide dialog */
  hide(): Promise<void>;
  /** Set content */
  setContent(content: DialogContent): void;
  /** Get result */
  getResult(): unknown;
}

/**
 * Dialog options
 */
export interface DialogOptions {
  /** Title */
  title?: string;
  /** Content */
  content?: DialogContent;
  /** Button configuration */
  buttons?: DialogButton[];
  /** Modal */
  modal?: boolean;
  /** Size */
  size?: { width: number; height: number };
}

/**
 * Dialog content
 */
export type DialogContent =
  | { type: 'text'; data: string }
  | { type: 'custom'; renderer: WindowContentRenderer };

/**
 * Dialog button
 */
export interface DialogButton {
  label: string;
  style?: 'primary' | 'secondary' | 'danger';
  onClick: () => void | Promise<void>;
}

/**
 * Menu component
 */
export interface MenuComponent {
  /** Show menu */
  show(x: number, y: number): Promise<void>;
  /** Hide menu */
  hide(): Promise<void>;
  /** Update menu items */
  updateItems(items: MenuItem[]): void;
}

/**
 * Menu item
 */
export interface MenuItem {
  /** Label */
  label: string;
  /** Icon */
  icon?: Buffer | string;
  /** Disabled */
  disabled?: boolean;
  /** Separator */
  separator?: boolean;
  /** Submenu */
  submenu?: MenuItem[];
  /** Click event */
  onClick?: () => void;
}

/**
 * Menu options
 */
export interface MenuOptions {
  /** Menu style */
  style?: 'default' | 'compact';
}

/**
 * Layout component
 */
export interface LayoutComponent {
  /** Add child component */
  addChild(component: Component): void;
  /** Remove child component */
  removeChild(component: Component): void;
  /** Set spacing */
  setSpacing(spacing: number): void;
  /** Set alignment */
  setAlign(align: 'start' | 'center' | 'end' | 'stretch'): void;
}

/**
 * Component base type
 */
export type Component =
  | ListComponent<unknown>
  | ButtonComponent
  | InputComponent
  | SelectComponent<unknown>
  | DialogComponent
  | MenuComponent
  | LayoutComponent;

// ============================================================================
// Component Builder (Builder Pattern, optional)
// ============================================================================

/**
 * Component Builder - Provides fluent API
 */
export class ComponentBuilder {
  /**
   * Create list
   */
  list<T>(items: T[]): ListBuilder<T>;

  /**
   * Create button
   */
  button(label: string): ButtonBuilder;

  /**
   * Create input
   */
  input(): InputBuilder;

  /**
   * Create select
   */
  select<T>(options: T[]): SelectBuilder<T>;
}

/**
 * List builder
 */
export interface ListBuilder<T> {
  render(renderer: ListItemRenderer<T>): ListBuilder<T>;
  virtual(enabled: boolean): ListBuilder<T>;
  selectable(enabled: boolean): ListBuilder<T>;
  build(): ListComponent<T>;
}

/**
 * Button builder
 */
export interface ButtonBuilder {
  style(style: ButtonOptions['style']): ButtonBuilder;
  size(size: ButtonOptions['size']): ButtonBuilder;
  icon(icon: Buffer | string): ButtonBuilder;
  onClick(handler: () => void): ButtonBuilder;
  build(): ButtonComponent;
}

/**
 * Input builder
 */
export interface InputBuilder {
  placeholder(text: string): InputBuilder;
  type(type: InputOptions['type']): InputBuilder;
  maxLength(length: number): InputBuilder;
  onChange(handler: (value: string) => void): InputBuilder;
  build(): InputComponent;
}

/**
 * Select builder
 */
export interface SelectBuilder<T> {
  placeholder(text: string): SelectBuilder<T>;
  searchable(enabled: boolean): SelectBuilder<T>;
  getLabel(fn: (item: T) => string): SelectBuilder<T>;
  onChange(handler: (value: T) => void): SelectBuilder<T>;
  build(): SelectComponent<T>;
}

// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Create UI context
 */
export function createUIContext(window: FloatingWindow): UIContext;

/**
 * Get component builder
 */
export function getComponentBuilder(): ComponentBuilder;
```

---

## Implementation Plan

### Phase 1: Basic Features (egui + winit)

#### Phase 1.1: Screenshot Tool Basics
- [x] Implement basic screenshot functionality (fullscreen/region)
- [ ] Implement interactive region selection
- [ ] Implement window snapping
- [ ] Implement selection adjustment (drag edge to resize)
- [ ] Implement selection change callback
- [x] Implement pixel color picker
- [ ] Basic annotation (brush, rectangle, arrow)
- [x] Implement screenshot save (multiple formats)

#### Phase 1.2: Long Screenshot
- [ ] Implement vertical scrolling screenshot
- [ ] Implement image stitching algorithm
- [ ] Implement smart deduplication
- [ ] Progress callback support

#### Phase 1.3: Floating Window Basics
- [ ] Implement window creation and management
- [ ] Implement window dragging
- [ ] Implement window shapes (rectangle, circle, custom)
- [ ] Implement custom shape window transparent area mouse pass-through
- [ ] Implement image display
- [ ] Implement window icon support (Emoji, preset icons)
- [ ] Implement edge particle effects (6 types: Rotating Halo, Pulse Ripple, Flowing Light, Stardust Scatter, Electric Spark, Smoke Wisp)
- [ ] Basic animation (fade in/out)

#### Phase 1.4: Clipboard Management Basics
- [ ] Implement clipboard listening
- [ ] Implement history storage
- [ ] Implement basic query and search
- [ ] Implement history UI

#### Phase 1.5: Basic Component Library
- [ ] Implement list component
- [ ] Implement button component
- [ ] Implement input component
- [ ] Implement basic layout component

### Phase 2: Advanced Features (egui + wgpu)

#### Phase 2.1: Advanced Annotation
- [ ] Implement more annotation tools (blur, highlight, etc.)
- [ ] Implement annotation layer management
- [ ] Implement undo/redo

#### Phase 2.2: Advanced Window Effects
- [ ] Integrate wgpu rendering backend
- [ ] Complete edge particle effects (GPU accelerated versions of all 6 effects)
- [ ] Implement complex animation effects
- [ ] Implement custom rendering
- [ ] Custom shape window performance optimization
- [ ] Performance optimization

#### Phase 2.3: Clipboard Advanced Features
- [ ] Implement tagging system
- [ ] Implement statistics
- [ ] Implement import/export
- [ ] Sensitive info filtering

#### Phase 2.4: Component Library Extension
- [ ] Implement more components (dialog, menu, etc.)
- [ ] Implement virtual scrolling
- [ ] Implement theme system
- [ ] Performance optimization

---

## Architecture Design Considerations

### 1. API Design Principles
- **Type Safety**: Complete TypeScript type definitions
- **Async First**: All time-consuming operations use Promise
- **Event Driven**: Support event listening and callbacks
- **Procedural Development**: Provide procedural API per roadmap requirements
- **Extensibility**: Support custom rendering and extensions

### 2. Performance Considerations
- **Async Operations**: All I/O operations async
- **Resource Management**: Timely release of windows and resources
- **Memory Management**: Use streaming for large images
- **Rendering Optimization**: Virtual scrolling, on-demand rendering

### 3. Error Handling
- **Unified Error Types**: Define standard error interface
- **Error Recovery**: Key operations support retry
- **Error Logging**: Complete error logging

### 4. Extensibility
- **Plugin System**: Support custom annotation tools
- **Theme System**: Support UI theme customization
- **Internationalization**: Support multiple languages (future)

---

## Dependencies Summary

### Phase 1 Dependencies (egui + winit)
```toml
[package]
name = "bot"
version = "0.1.3"
edition = "2024"  # Using Rust 2024 Edition
rust-version = "1.85"  # Rust 2024 Edition requirement

[dependencies]
# Window management
winit = "0.31"  # Latest version
raw-window-handle = "0.6"  # Latest version

# UI framework
egui = "0.28"  # Latest version
egui-winit = "0.28"  # Latest version

# Rendering backend
softbuffer = "0.5"  # Latest version, CPU rendering

# Image processing
image = "0.25"  # Latest version
imageproc = "0.25"  # Latest version, image processing algorithms

# Icon support
# Option 1: Use emoji (system built-in, no extra dependency, recommended for simple scenarios)
# Option 2: Built-in SVG icon library (recommended, package common icons as SVG, render with resvg)
resvg = "0.43"  # Latest version, SVG rendering library
# Option 3: Font icon library (e.g., fontdue, for font rendering)
fontdue = "0.8"  # Latest version, font rendering (optional, for icons and text)

# Clipboard
arboard = "3.5"  # Latest version

# Database (clipboard history)
rusqlite = { version = "0.32", features = ["bundled"] }  # Latest version

# Time handling
chrono = { version = "0.4", features = ["serde"] }  # Latest version

# Async runtime
tokio = { version = "1.40", features = ["full"] }  # Latest version

# Serialization
serde = { version = "1.0", features = ["derive"] }  # Latest version
serde_json = "1.0"  # Latest version

# Random number generation (for particle effects)
rand = "0.8"  # Latest version

# Noise functions (for particle effect random paths)
noise = "0.9"  # Latest version, Perlin noise etc.

# Node.js bindings
napi = { version = "3.3", default-features = false, features = ["async"] }  # Latest version
napi-derive = "3.3"  # Latest version
```

### Phase 2 Dependencies (egui + wgpu)
```toml
[dependencies]
# Add on top of Phase 1:
wgpu = "0.21"  # Latest version
egui-wgpu = "0.28"  # Latest version, matches egui version
```

---

## Notes

1. **API Stability**: Phase 1 API design should consider Phase 2 extension, avoid breaking changes
2. **Performance Testing**: Each feature module needs performance testing
3. **Cross-platform Compatibility**: Ensure Windows, macOS, Linux all work properly
4. **Documentation**: Each API needs complete JSDoc comments and examples
5. **Test Coverage**: Key features need unit tests and integration tests

---

## Future Optimization Directions

1. **Performance Optimization**: Virtual scrolling, image compression, caching strategies
2. **User Experience**: Hotkey support, gesture support, theme customization
3. **Feature Extension**: OCR integration, image recognition, automation scripts
4. **Cloud Sync**: Clipboard history cloud sync (optional)
5. **Plugin System**: Support third-party plugin extensions
