/** @jsx jsx */
import { jsx } from "theme-ui"
import SectionWrapper from "./section-wrapper"
// @ts-ignore
import JobsMDX from "../sections/jobs"

const Jobs = ({ offset, factor = 2 }: { offset: number; factor?: number }) => (
  <SectionWrapper offset={offset} factor={factor} slantDirection="right">
    <JobsMDX />
  </SectionWrapper>
)

export default Jobs

