// README / social banner: pixel pouch logo + name + tagline, characters as illustration on the right.
import { AbsoluteFill, Img, staticFile } from "remotion";
import { loadFont } from "@remotion/google-fonts/DotGothic16";
import type { Lang } from "./Teaser";

const { fontFamily: PIXEL } = loadFont();
const BODY = '"PingFang SC","Hiragino Sans GB","Noto Sans SC",sans-serif';
const TAG: Record<Lang, [string, string]> = {
  zh: ["懂你的品味，帮你发现值得喜欢的东西。", "把记忆变成卡 · 按兴趣派 agent · 本机运行"],
  en: ["Knows your taste. Finds the next thing worth loving.", "Memory as cards · agents that follow your interests · runs locally"],
  ja: ["あなたの好みを理解し、次の「好き」を見つける。", "記憶をカードに · 興味でエージェントが動く · ローカルで動く"],
};

export const Banner: React.FC<{ lang: Lang; male: boolean }> = ({ lang, male }) => {
  const m = male ? "-m" : "";
  return (
    <AbsoluteFill style={{ background: "#f6ead3" }}>
      <AbsoluteFill style={{ backgroundImage: "radial-gradient(#e6cfa6 1.5px, transparent 1.6px)", backgroundSize: "26px 26px", opacity: 0.6 }} />
      <div style={{ position: "absolute", left: 0, right: 0, top: 0, height: 14, background: "#e8762c", boxShadow: "0 4px 0 #b4561c" }} />
      <div style={{ position: "absolute", left: 60, top: 96, display: "flex", gap: 28, alignItems: "center" }}>
        <Img src={staticFile("logo.png")} style={{ width: 130, height: 130, imageRendering: "pixelated" }} />
        <div>
          <div style={{ fontFamily: PIXEL, fontSize: 70, color: "#3a2416", letterSpacing: 4, lineHeight: 1.1 }}>Sense 口袋</div>
          <div style={{ fontFamily: BODY, fontWeight: 700, color: "#b4561c", marginTop: 8, maxWidth: 500, fontSize: 26 }}>{TAG[lang][0]}</div>
          <div style={{ fontFamily: PIXEL, color: "#8a6a4e", marginTop: 10, letterSpacing: 1, maxWidth: 500, fontSize: 17 }}>{TAG[lang][1]}</div>
        </div>
      </div>
      <Img src={staticFile(`c/claude${m}/wave.png`)} style={{ position: "absolute", right: 270, bottom: -30, width: 260, height: 260 }} />
      <Img src={staticFile(`c/deepseek${m}/idle.png`)} style={{ position: "absolute", right: -10, bottom: -30, width: 260, height: 260 }} />
      <Img src={staticFile(`c/pouch${m}/cheer.png`)} style={{ position: "absolute", right: 140, bottom: -24, width: 215, height: 215 }} />
    </AbsoluteFill>
  );
};
