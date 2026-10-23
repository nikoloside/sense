import { Composition } from "remotion";
import { Teaser, TEASER_FRAMES } from "./Teaser";

export const Root = () => (
  <Composition id="Teaser" component={Teaser} durationInFrames={TEASER_FRAMES} fps={30} width={1280} height={720} />
);
