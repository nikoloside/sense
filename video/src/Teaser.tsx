// Sense teaser for the download page (ludics.space). Characters are the desk mascots (docs/art/characters.md);
// the logo is the pixel pouch icon — characters never stand in for the icon.
import { AbsoluteFill, Img, Sequence, interpolate, spring, staticFile, useCurrentFrame, useVideoConfig, Easing } from "remotion";
import { loadFont } from "@remotion/google-fonts/DotGothic16";

const { fontFamily: PIXEL } = loadFont();
const BODY = '"PingFang SC","Hiragino Sans GB","Noto Sans SC",sans-serif';
const C = { bg: "#f6ead3", panel: "#fff7e8", line: "#e6cfa6", ink: "#3a2416", ink2: "#8a6a4e", ac: "#e8762c", ac2: "#b4561c", gold: "#e3a72f", silver: "#9fb0bf", bronze: "#c4773b", stone: "#a39c92" };

export const TEASER_FRAMES = 690;

type Who = "claude" | "deepseek" | "pouch";
const frame = (who: Who, f: string) => staticFile(`c/${who}/${f}.png`);

/** A character that blinks on its own; `pose` picks the frame, `talk` flips mouth frames. */
const Char: React.FC<{ who: Who; pose?: string; talk?: boolean; size?: number; style?: React.CSSProperties }> = ({ who, pose = "idle", talk, size = 380, style }) => {
  const f = useCurrentFrame();
  let name = pose;
  if (talk) name = ["idle_talk_a", "idle_talk_b", "idle_talk_a", "idle"][Math.floor(f / 4) % 4];
  else if ((pose === "idle" || pose === "busy") && f % 75 > 70) name = pose + "_blink";
  return <Img src={frame(who, name)} style={{ width: size, height: size, position: "absolute", ...style }} />;
};

const pop = (f: number, fps: number, delay = 0) => spring({ frame: f - delay, fps, config: { damping: 12, stiffness: 140 } });
const fade = (f: number, a: number, b: number) => interpolate(f, [a, a + 12, b - 12, b], [0, 1, 1, 0], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });

const Caption: React.FC<{ zh: string; en: string; f: number; from: number; to: number }> = ({ zh, en, f, from, to }) => (
  <div style={{ position: "absolute", left: 80, bottom: 70, opacity: fade(f, from, to), transform: `translateY(${interpolate(f, [from, from + 12], [12, 0], { extrapolateRight: "clamp", extrapolateLeft: "clamp" })}px)` }}>
    <div style={{ fontFamily: BODY, fontWeight: 700, fontSize: 40, color: C.ink, letterSpacing: 1 }}>{zh}</div>
    <div style={{ fontFamily: PIXEL, fontSize: 20, color: C.ink2, letterSpacing: 2, marginTop: 6 }}>{en}</div>
  </div>
);

const TIER = { stone: ["石", C.stone], bronze: ["铜", C.bronze], silver: ["银", C.silver], gold: ["金", C.gold] } as const;
type Tier = keyof typeof TIER;

const Card: React.FC<{ title: string; kind: string; tier: Tier; style?: React.CSSProperties; glow?: number }> = ({ title, kind, tier, style, glow = 0 }) => (
  <div style={{ position: "absolute", width: 170, height: 230, borderRadius: 14, background: C.panel, border: `5px solid ${TIER[tier][1]}`, boxShadow: `0 6px 0 ${TIER[tier][1]}, 0 0 ${40 * glow}px ${TIER[tier][1]}`, padding: 14, boxSizing: "border-box", display: "flex", flexDirection: "column", gap: 8, ...style }}>
    <div style={{ fontFamily: PIXEL, fontSize: 18, color: TIER[tier][1] }}>{TIER[tier][0]} · {kind}</div>
    <div style={{ flex: 1, borderRadius: 8, background: `repeating-linear-gradient(45deg, ${TIER[tier][1]}22 0 8px, transparent 8px 16px)` }} />
    <div style={{ fontFamily: BODY, fontWeight: 700, fontSize: 20, color: C.ink, lineHeight: 1.25 }}>{title}</div>
  </div>
);

const Bg: React.FC = () => (
  <AbsoluteFill style={{ background: C.bg }}>
    <AbsoluteFill style={{ backgroundImage: `radial-gradient(${C.line} 1.5px, transparent 1.6px)`, backgroundSize: "26px 26px", opacity: 0.6 }} />
    <div style={{ position: "absolute", left: 0, right: 0, top: 0, height: 14, background: C.ac, boxShadow: `0 4px 0 ${C.ac2}` }} />
  </AbsoluteFill>
);

/* 1 · logo + 口袋精 waves */
const Intro: React.FC = () => {
  const f = useCurrentFrame(); const { fps } = useVideoConfig();
  const s = pop(f, fps); const t = pop(f, fps, 10); const p = pop(f, fps, 22);
  return (
    <AbsoluteFill>
      <Img src={staticFile("logo.png")} style={{ position: "absolute", left: 150, top: 170, width: 200, height: 200, imageRendering: "pixelated", transform: `scale(${s})` }} />
      <div style={{ position: "absolute", left: 380, top: 190, opacity: t, transform: `translateX(${(1 - t) * -30}px)` }}>
        <div style={{ fontFamily: PIXEL, fontSize: 92, color: C.ink, letterSpacing: 4 }}>Sense 口袋</div>
        <div style={{ fontFamily: BODY, fontSize: 32, color: C.ac2, marginTop: 4, fontWeight: 700 }}>懂你的品味，帮你发现值得喜欢的东西。</div>
        <div style={{ fontFamily: PIXEL, fontSize: 22, color: C.ink2, marginTop: 10, letterSpacing: 3 }}>YOUR PERSONAL TASTE AGENT</div>
      </div>
      <Char who="pouch" pose="wave" size={330} style={{ right: 40, bottom: -20, transform: `translateY(${(1 - p) * 300}px)` }} />
    </AbsoluteFill>
  );
};

/* 2 · things you saw / heard / grabbed / said fly into the pouch */
const ITEMS: { title: string; kind: string; d: number }[] = [
  { title: "一张截图", kind: "截图", d: 0 },
  { title: "常听的那首歌", kind: "音乐", d: 22 },
  { title: "追到一半的番", kind: "动画", d: 44 },
  { title: "和 AI 聊过的判断", kind: "对话", d: 66 },
];
const Intake: React.FC = () => {
  const f = useCurrentFrame();
  const eating = ITEMS.some((it) => f - it.d > 26 && f - it.d < 44);
  return (
    <AbsoluteFill>
      {ITEMS.map((it, i) => {
        const k = interpolate(f - it.d, [0, 34], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp", easing: Easing.in(Easing.quad) });
        const x = interpolate(k, [0, 1], [110 + i * 40, 980]);
        const y = interpolate(k, [0, 1], [150 + i * 30, 470]) - Math.sin(k * Math.PI) * 140;
        return <Card key={i} title={it.title} kind={it.kind} tier="stone" style={{ left: x, top: y, opacity: f < it.d ? 0 : 1 - k * 0.9, transform: `scale(${1 - k * 0.6}) rotate(${k * 30 - 8}deg)` }} />;
      })}
      <Char who="pouch" pose={eating ? "eat" : "idle"} size={360} style={{ right: 30, bottom: -20 }} />
      <Caption f={f} from={0} to={150} zh="看过、听过、截过、聊过的，变成一张张卡" en="WHAT YOU WATCH, HEAR, GRAB AND SAY → CARDS" />
    </AbsoluteFill>
  );
};

/* 3 · tiers, and a stone card that shatters */
const Tiers: React.FC = () => {
  const f = useCurrentFrame(); const { fps } = useVideoConfig();
  const order: [Tier, string, string][] = [
    ["stone", "石 · 刚进来", "STONE · JUST IN"],
    ["bronze", "铜 · 你留下了", "BRONZE · KEPT"],
    ["silver", "银 · 在关注，有新的就告诉你", "SILVER · FOLLOWED"],
    ["gold", "金 · 两张银卡说得出同一个为什么", "GOLD · A TASTE"],
  ];
  const step = Math.min(3, Math.floor(f / 26));
  const crackAt = 112;
  const shake = f > crackAt && f < crackAt + 14 ? Math.sin(f * 3) * 8 : 0;
  const shard = interpolate(f, [crackAt + 10, crackAt + 30], [0, 1], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
  return (
    <AbsoluteFill>
      {order.map(([t], i) => {
        const s = pop(f, fps, i * 26);
        const isStone = t === "stone";
        return (
          <div key={t} style={{ position: "absolute", left: 90 + i * 200, top: 140, transform: `scale(${s}) translateX(${isStone ? shake : 0}px)`, opacity: isStone ? 1 - shard : 1 }}>
            <Card title={["没理会的那张", "留下的那张", "关注的那位", "你的一种口味"][i]} kind={["7 天后碎", "在意", "追更", "往外推荐"][i]} tier={t} style={{ position: "relative", transform: `rotate(${isStone ? shard * -20 : 0}deg) translateY(${isStone ? shard * 120 : 0}px)` }} glow={i === step && f < crackAt ? 0.6 : 0} />
          </div>
        );
      })}
      <Char who="pouch" pose={f > crackAt && f < crackAt + 30 ? "crack" : f > 80 && f < crackAt ? "cheer" : "idle"} size={330} style={{ right: 20, bottom: -20 }} />
      {f < crackAt ? <Caption f={f} from={0} to={crackAt} zh={order[step][1]} en={order[step][2]} /> : <Caption f={f} from={crackAt} to={150} zh="7 天不理会就碎掉，只留下「查到过」" en="IGNORED FOR 7 DAYS? IT SHATTERS" />}
    </AbsoluteFill>
  );
};

/* 4 · the engine: Claude or DeepSeek goes looking */
const Engine: React.FC = () => {
  const f = useCurrentFrame(); const { fps } = useVideoConfig();
  const a = pop(f, fps); const b = pop(f, fps, 12); const n = pop(f, fps, 40);
  return (
    <AbsoluteFill>
      <Char who="claude" pose="busy" size={430} style={{ right: 300, bottom: -20, transform: `translateX(${(1 - a) * 700}px)` }} />
      <Char who="deepseek" pose="busy" size={430} style={{ right: 20, bottom: -20, transform: `translateX(${(1 - b) * 500}px)` }} />
      <div style={{ position: "absolute", left: 80, top: 110, width: 420, background: C.panel, border: `4px solid ${C.silver}`, borderRadius: 14, padding: "18px 22px", boxShadow: `0 5px 0 ${C.silver}`, transform: `scale(${n})`, transformOrigin: "top left" }}>
        <div style={{ fontFamily: PIXEL, fontSize: 18, color: C.ink2 }}>追踪台 · 刚查到</div>
        <div style={{ fontFamily: BODY, fontSize: 26, fontWeight: 700, color: C.ink, marginTop: 6 }}>你关注的那位出新作了</div>
        <div style={{ fontFamily: BODY, fontSize: 18, color: C.ink2, marginTop: 4 }}>来源链接 · 日期 · 一句为什么</div>
      </div>
      <Caption f={f} from={0} to={120} zh="替你去查新一话、新作" en="ENGINE · CLAUDE OR DEEPSEEK" />
    </AbsoluteFill>
  );
};

/* 5 · voice: call the one on your desk */
const Voice: React.FC = () => {
  const f = useCurrentFrame(); const { fps } = useVideoConfig();
  const s = pop(f, fps); const bub = pop(f, fps, 14);
  const text = "哼，本小姐都替你记下了。";
  const shown = text.slice(0, Math.max(0, Math.floor((f - 14) / 2.5)));
  return (
    <AbsoluteFill>
      <Char who="claude" talk={f > 14 && f < 70} pose="idle" size={460} style={{ right: 60, bottom: -20, transform: `scale(${s})`, transformOrigin: "bottom center" }} />
      <div style={{ position: "absolute", right: 380, top: 150, background: "#fff", border: `3px solid ${C.ink}`, borderRadius: 16, padding: "14px 22px", boxShadow: `0 4px 0 ${C.ink}`, fontFamily: BODY, fontSize: 28, color: C.ink, transform: `scale(${bub})`, transformOrigin: "bottom right", minWidth: 40 }}>{shown || "…"}</div>
      <Caption f={f} from={0} to={90} zh="直接打电话，聊你口袋里的东西" en="TALK TO IT · 中文 / 日本語 / ENGLISH" />
    </AbsoluteFill>
  );
};

/* 6 · everyone cheers, badges, url */
const BADGES = ["macOS", "本机运行", "不上传", "开源 MIT", "Claude · DeepSeek", "中 / 日 / 英"];
const Outro: React.FC = () => {
  const f = useCurrentFrame(); const { fps } = useVideoConfig();
  return (
    <AbsoluteFill>
      {(["claude", "pouch", "deepseek"] as Who[]).map((w, i) => {
        const s = pop(f, fps, i * 6);
        return <Char key={w} who={w} pose="cheer" size={w === "pouch" ? 300 : 340} style={{ left: 330 + i * 230 - (w === "pouch" ? -20 : 0), bottom: -20, transform: `translateY(${(1 - s) * 300}px)` }} />;
      })}
      <div style={{ position: "absolute", left: 0, right: 0, top: 70, display: "flex", flexDirection: "column", alignItems: "center", gap: 18 }}>
        <div style={{ display: "flex", alignItems: "center", gap: 18, opacity: pop(f, fps, 4) }}>
          <Img src={staticFile("logo.png")} style={{ width: 84, height: 84, imageRendering: "pixelated" }} />
          <div style={{ fontFamily: PIXEL, fontSize: 64, color: C.ink, letterSpacing: 3 }}>Sense 口袋</div>
        </div>
        <div style={{ display: "flex", gap: 10, flexWrap: "wrap", justifyContent: "center", maxWidth: 980 }}>
          {BADGES.map((b, i) => (
            <div key={b} style={{ fontFamily: BODY, fontWeight: 700, fontSize: 22, color: i % 2 ? C.ac2 : "#fff", background: i % 2 ? C.panel : C.ac, border: `3px solid ${C.ac2}`, borderRadius: 999, padding: "6px 16px", transform: `scale(${pop(f, fps, 14 + i * 4)})` }}>{b}</div>
          ))}
        </div>
        <div style={{ fontFamily: PIXEL, fontSize: 34, color: C.ac2, letterSpacing: 3, opacity: pop(f, fps, 40) }}>ludics.space</div>
      </div>
    </AbsoluteFill>
  );
};

const SCENES: [React.FC, number][] = [[Intro, 90], [Intake, 150], [Tiers, 150], [Engine, 120], [Voice, 90], [Outro, 90]];

export const Teaser: React.FC = () => {
  let at = 0;
  return (
    <AbsoluteFill>
      <Bg />
      {SCENES.map(([S, d], i) => {
        const from = at; at += d;
        return (
          <Sequence key={i} from={from} durationInFrames={d}>
            <SceneFade d={d} first={i === 0}><S /></SceneFade>
          </Sequence>
        );
      })}
    </AbsoluteFill>
  );
};

const SceneFade: React.FC<{ d: number; first: boolean; children: React.ReactNode }> = ({ d, first, children }) => {
  const f = useCurrentFrame();
  const o = interpolate(f, [0, 8, d - 8, d], [first ? 1 : 0, 1, 1, 0], { extrapolateLeft: "clamp", extrapolateRight: "clamp" });
  return <AbsoluteFill style={{ opacity: o }}>{children}</AbsoluteFill>;
};
