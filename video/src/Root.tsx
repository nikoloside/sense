import { Composition } from "remotion";
import { Teaser, TEASER_FRAMES, TeaserProps } from "./Teaser";
import { Banner } from "./Banner";

const defaults: TeaserProps = { lang: "zh", male: false };

export const Root = () => (
  <>
    <Composition id="Teaser" component={Teaser} durationInFrames={TEASER_FRAMES} fps={30} width={1280} height={720} defaultProps={defaults} />
    <Composition id="Banner" component={Banner} durationInFrames={1} fps={30} width={1280} height={400} defaultProps={defaults} />
  </>
);
