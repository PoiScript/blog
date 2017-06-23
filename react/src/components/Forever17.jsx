import React from 'react'

const millisecondsInADay = 8.64e+7

/**
 * convert your age to '17 岁 + N 天' format
 */

class Forever17 extends React.Component {
  componentDidMount () {
    const birthday = new Date(this.props.birthday)
    const seventeenYearsAgo = new Date().setFullYear(new Date().getFullYear() - 17)
    this.setState = {
      days: (seventeenYearsAgo - birthday) / millisecondsInADay
    }
  }

  render () {
    return (
      <span>17 岁 + {this.state ? Math.floor(this.state.days) : 'N'} 天</span>
    )
  }
}

export default Forever17
