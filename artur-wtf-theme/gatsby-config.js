module.exports = ({ basePath = `/`, mdx = true }) => ({
  siteMetadata: {
    siteTitle: `Artur Das`,
    siteTitleAlt: `Artur artur-wtf-theme`,
    siteHeadline: `Artur Das - Gatsby Theme from @artur-wtf`,
    siteUrl: `https://artur.wtf`,
    siteDescription: `Playful and Colorful One-Page portfolio featuring Parallax effects and animations`,
    siteLanguage: `en`,
    siteImage: `/Platypus.svg`,
    siteIcon: `/Platypus.svg`,
    author: `@arty_das`,
    basePath,
  },
  plugins: [
    {
      resolve: `gatsby-plugin-google-analytics`,
      options: {
        anonymize: true,
        trackingId: "UA-118770025-2",
        respectDNT: true,
        cookieDomain: "artur.wtf",
      },
    },
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `sections`,
        path: `${__dirname}/src/sections`,
      },
    },
    mdx && {
      resolve: `gatsby-plugin-mdx`,
      options: {},
    },
    `gatsby-plugin-react-helmet`,
    `gatsby-plugin-typescript`,
    `gatsby-plugin-emotion`,
    `gatsby-plugin-theme-ui`,
  ],
})
