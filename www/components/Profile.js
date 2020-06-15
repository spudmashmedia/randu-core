/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Spudmash Media Pty Ltd
 *  Licensed under the MIT License. See License.md in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import React from "react";
import { makeStyles } from "@material-ui/core/styles";
import CardActionArea from "@material-ui/core/CardActionArea";
import Card from "@material-ui/core/Card";
import CardContent from "@material-ui/core/CardContent";
import CardMedia from "@material-ui/core/CardMedia";
import Typography from "@material-ui/core/Typography";

const useStyles = makeStyles((theme) => ({
  root: {
    maxWidth: 350,
  },
  media: {
    height: 150,
  },
}));

export default function Profile(props) {
  let {
    data: { name, username, password, location, image_lrg },
  } = props;

  const classes = useStyles();

  return (
    <Card className={classes.root}>
      <CardActionArea>
        <CardMedia className={classes.media} image={image_lrg} title={name} />
      </CardActionArea>
      <CardContent>
        <Typography variant="h6">{name}</Typography>
        <Typography>Username: {username}</Typography>
        <Typography>Password: {password}</Typography>
        <Typography>Country: {location}</Typography>
      </CardContent>
    </Card>
  );
}
